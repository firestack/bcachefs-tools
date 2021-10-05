{ lib

, stdenv
, glibc
, llvmPackages
, rustPlatform

, bcachefs
, pkg-config

, udev
, liburcu
, zstd
, keyutils
, libaio
		
, lz4 # liblz4
, libsodium
, libuuid
, zlib # zlib1g
, libscrypt

, ...
}: let 
	include = {
		glibc = "${stdenv.glibc.dev}/include";
		clang = let libc = llvmPackages.libclang; in
			"${libc.lib}/lib/clang/${libc.version}/include";
		urcu = "${liburcu}/include";
		zstd = "${zstd.dev}/include";
	};
in rustPlatform.buildRustPackage ( let 
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
	nativeBuildInputs = [ pkg-config ];
	buildInputs = [
		# libaio
		keyutils # libkeyutils
		lz4 # liblz4
		libsodium
		liburcu
		libuuid
		zstd # libzstd
		zlib # zlib1g
		udev
		libscrypt
		libaio
	];
	
	LIBBCACHEFS_LIB ="${bcachefs.tools}/lib";
	LIBBCACHEFS_INCLUDE = bcachefs.tools.src;
	LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
	BINDGEN_EXTRA_CLANG_ARGS = lib.replaceStrings ["\n" "\t"] [" " ""] ''
		-std=gnu99
		-H
		-I${include.glibc}
		-I${include.clang}
		-I${include.urcu}
		-I${include.zstd}
	'';

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