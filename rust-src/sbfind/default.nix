{ lib

, stdenv
, glibc
, llvmPackages
, rustPlatform

, bcachefs
, self

, ...
}: rustPlatform.buildRustPackage ( let 
	cargo = lib.trivial.importTOML ./Cargo.toml;
in {
	pname = cargo.package.name;
	version = cargo.package.version;
	
	src = bcachefs.rlibbcachefs.src;
	# srcs = let src = builtins.path {path = ../../.; name = "${cargo.package.name}.bcachefs-srcs";}; in map (i: src + i) [ 
	# 	"/cmds"
	# 	"/rlibbcachefs"
	# ];
	sourceRoot = "rlibbcachefs/cmds/sbfind";


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
	
	# -isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.getVersion llvmPackages.libclang}/include";
	# CFLAGS = "-I${llvmPackages.libclang.lib}/include";
	# LDFLAGS = "-L${libcdev}";

	doCheck = false;
	# NIX_DEBUG = 6;
})