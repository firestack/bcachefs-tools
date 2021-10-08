{ lib

, stdenv
, glibc
, llvmPackages
, rustPlatform

, bcachefs

, ...
}: rustPlatform.buildRustPackage ( let 
	cargo = lib.trivial.importTOML ./Cargo.toml;
in {
	pname = "mount.bcachefs";
	version = cargo.package.version;
	
	srcs = let src = builtins.path {path = ../.; name = "mount.bcachefs-srcs";}; in map (i: src + i) [ 
		"/mount"
		"/rlibbcachefs"
	];
	sourceRoot = "mount/";


	cargoLock = { lockFile = ./Cargo.lock; };

	nativeBuildInputs = bcachefs.rlibbcachefs.nativeBuildInputs;
	buildInputs = bcachefs.rlibbcachefs.buildInputs;
	inherit (bcachefs.rlibbcachefs)
		LIBBCACHEFS_INCLUDE
		LIBBCACHEFS_LIB
		LIBCLANG_PATH
		BINDGEN_EXTRA_CLANG_ARGS;

	postPatch = ''
		cp ${./Cargo.lock} Cargo.lock
	'';
	
	preFixup = ''
		ln $out/bin/mount-bcachefs $out/bin/mount.bcachefs
		ln -s $out/bin $out/sbin
	'';
	# -isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.getVersion llvmPackages.libclang}/include";
	# CFLAGS = "-I${llvmPackages.libclang.lib}/include";
	# LDFLAGS = "-L${libcdev}";

	doCheck = false;
	
	# NIX_DEBUG = 4;
})