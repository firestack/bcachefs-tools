{ lib

, stdenv
, pkg-config
, attr
, libuuid
, libsodium
, keyutils

, liburcu
, zlib
, libaio
, udev
, zstd
, lz4

, python39
, python39Packages
, docutils
, nixosTests

, bcachefs
, lastModified
, versionString ? lastModified

, inShell ? false
, debugMode ? inShell

, testWithValgrind ? true
, valgrind 

, fuseSupport ? false
, fuse3 ? null }:

assert fuseSupport -> fuse3 != null;
assert testWithValgrind -> valgrind != null;
let version = "v0.1-flake-${versionString}"; in
stdenv.mkDerivation {
	pname = "bcachefs-tools";

	inherit version;
	VERSION = version;
	
	src = bcachefs.toolsSrc;

	postPatch = "patchShebangs --build doc/macro2rst.py";

	RS_INCLUDE="${bcachefs.rbcachefs}/include";
	preBuild = ''
		cp --reflink=auto ${bcachefs.rbcachefs}/lib/librbcachefs.a .
	'';
	nativeBuildInputs = [
		# used to find dependencies
		## see ./INSTALL
		pkg-config
	];
	buildInputs = [
		# bcachefs explicit dependencies
		## see ./INSTALL
		libaio
		
		# libblkid
		keyutils # libkeyutils
		lz4 # liblz4
		
		libsodium
		liburcu
		libuuid
		zstd # libzstd
		zlib # zlib1g
		valgrind

		# unspecified dependencies
		attr
		udev

		# documentation depenedencies
		docutils
		python39Packages.pygments
	] ++ (lib.optional fuseSupport fuse3)
	++ (lib.optional testWithValgrind valgrind) ;

	makeFlags = [
		"PREFIX=${placeholder "out"}"
		"EXTRA_CFLAGS=-gdwarf -Wl,--build-id"

	] ++ lib.optional debugMode "";

	installFlags = [
		"INITRAMFS_DIR=${placeholder "out"}/etc/initramfs-tools"
	];

	doCheck = true; # needs bcachefs module loaded on builder

	checkInputs = [
		python39Packages.pytest
		python39Packages.pytest-xdist
	] ++ lib.optional testWithValgrind valgrind;
	
	checkFlags = [ 
		"BCACHEFS_TEST_USE_VALGRIND=${if testWithValgrind then "yes" else "no"}"
		# cannot escape spaces within make flags, quotes are stripped
		"PYTEST_CMD=pytest"
	];

	preCheck =
		''
			makeFlagsArray+=(PYTEST_ARGS="--verbose -n$NIX_BUILD_CORES")
		'' +
		lib.optionalString fuseSupport ''
			rm tests/test_fuse.py
		'';

	# dontStrip = debugMode == true;
	separateDebugInfo = true;

	passthru = {
		bcachefs_revision = let 
			file = builtins.readFile ./.bcachefs_revision;
			removeLineFeeds = str: lib.lists.foldr (lib.strings.removeSuffix) str ["\r" "\n"];
		in removeLineFeeds file;
		
		tests = {
			smoke-test = nixosTests.bcachefs;
		};
	};

	enableParallelBuilding = true;
	meta = with lib; {
		description = "Userspace tools for bcachefs";
		homepage    = http://bcachefs.org;
		license     = licenses.gpl2;
		platforms   = platforms.linux;
		maintainers =
			[ "Kent Overstreet <kent.overstreet@gmail.com>"
			];

	};
}
