{ lib
, filter

, stdenv
, pkg-config
, attr
, libuuid
, libscrypt
, libsodium
, keyutils

, liburcu
, zlib
, libaio
, udev
, zstd
, lz4
, valgrind

, python39
, python39Packages
, docutils
, nixosTests

, lastModified
, versionString ? lastModified

, inShell ? false
, debugMode ? inShell

, testWithValgrind ? true

, fuseSupport ? false
, fuse3 ? null }:

assert fuseSupport -> fuse3 != null;

stdenv.mkDerivation {
	pname = "bcachefs-tools";

	version = "v0.1-flake-${versionString}";
	VERSION = "v0.1-flake-${versionString}";
	
	src = filter.filter {
		name = "bcachefs-tools";
		root = ./.;
		exclude = [
			./rlibbcachefs
			
			./.git
			./nix
			
			./flake.nix
			./flake.lock
		];
	};

	postPatch = "patchShebangs --build doc/macro2rst.py";

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
		
		libscrypt
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
	] ++ lib.optional fuseSupport fuse3;

	makeFlags = [
		"PREFIX=${placeholder "out"}"
	] ++ lib.optional debugMode "EXTRA_CFLAGS=-ggdb";

	installFlags = [
		"INITRAMFS_DIR=${placeholder "out"}/etc/initramfs-tools"
	];

	dontStrip = debugMode;
	doCheck = true; # needs bcachefs module loaded on builder
	
	checkFlags = [ "BCACHEFS_TEST_USE_VALGRIND=${if testWithValgrind then "yes" else "no"}"];
		# cannnot escape spaces within make flags
		# "PYTEST=pytest\ --verbose\ -n4"
	checkInputs = [
		valgrind
		python39Packages.pytest
		python39Packages.pytest-xdist
	];

	preCheck =
		''
			makeFlagsArray+=(PYTEST="pytest --verbose -n4")
		'' +
		lib.optionalString fuseSupport ''
			rm tests/test_fuse.py
		'';

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
