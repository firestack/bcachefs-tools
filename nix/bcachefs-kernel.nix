{ lib
, fetchpatch
, fetchgit
, fetchFromGitHub
, buildLinux
, date ? "2021-08-05"
, commit ? "60fbf06f49679fdb2b37e1e863c321dfddfc3a4a"
, diffHash ? "sha256-9NUTmC8FnXJzJ0tF2FrGW10fuGSRVq3ONdSzVmoOSTs="
, kernelVersion ? "5.13.0"
, kernelPatches ? [] # must always be defined in bcachefs' all-packages.nix entry because it's also a top-level attribute supplied by callPackage
, argsOverride ? {}
, versionString ? (builtins.substring 0 8 commit)
, ...
} @ args:

buildLinux {
	inherit kernelPatches;

	# pname = "linux";
	version = "${kernelVersion}-bcachefs-${versionString}";
	
	modDirVersion = kernelVersion;
	

	src = fetchFromGitHub {
		owner = "koverstreet";
		repo = "bcachefs";
		rev = commit;
		sha256 = diffHash;
	};

	extraConfig = "BCACHEFS_FS m";
	# NIX_DEBUG=5;
}