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
	
	src = builtins.path { path = ../.; name = "rust-src"; };
	sourceRoot = "rust-src/mount";

	cargoLock = { lockFile = ./Cargo.lock; };

	nativeBuildInputs = bcachefs.bch_bindgen.nativeBuildInputs;
	buildInputs = bcachefs.bch_bindgen.buildInputs;
	inherit (bcachefs.bch_bindgen)
		LIBBCACHEFS_INCLUDE
		LIBBCACHEFS_LIB
		LIBCLANG_PATH
		BINDGEN_EXTRA_CLANG_ARGS;
	
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