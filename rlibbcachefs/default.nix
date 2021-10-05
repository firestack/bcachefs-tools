{ lib
, stdenv
, rustPlatform
, llvmPackages
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

, glibc
, ...
}: let 
	include = {
		glibc = "${glibc.dev}/include";
		clang = let libc = llvmPackages.libclang; in
			"${libc.lib}/lib/clang/${libc.version}/include";
		urcu = "${liburcu}/include";
		zstd = "${zstd.dev}/include";
	};
in rustPlatform.buildRustPackage {
	pname = "bcachefs-rs";
	version = "0.0.0";
	
	src = builtins.path { path = ./.; name = "rlibbcachefs"; };

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
	

	doCheck = true;
	
	# NIX_DEBUG = 4;
}