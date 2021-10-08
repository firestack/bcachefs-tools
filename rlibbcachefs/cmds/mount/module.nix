## Mirrors: https://github.com/NixOS/nixpkgs/blob/nixos-unstable/nixos/modules/tasks/filesystems/bcachefs.nix
## with changes to use flakes and import mount.bcachefs
{ config, lib, pkgs, utils, ... }:

with lib;

let

	bootFs = filterAttrs (n: fs: (fs.fsType == "bcachefs") && (utils.fsNeededForBoot fs)) config.fileSystems;

in

{
	config = mkIf (elem "bcachefs" config.boot.supportedFilesystems) (mkMerge [
		{
			system.fsPackages = [ pkgs.bcachefs.tools pkgs.bcachefs.mount ];

			# use kernel package with bcachefs support until it's in mainline
			boot.kernelPackages = pkgs.bcachefs.kernelPackages;
		}

		(mkIf ((elem "bcachefs" config.boot.initrd.supportedFilesystems) || (bootFs != {})) {
			# chacha20 and poly1305 are required only for decryption attempts
			boot.initrd.availableKernelModules = [ "bcachefs" "sha256" "chacha20" "poly1305" ];

			boot.initrd.extraUtilsCommands = ''
				copy_bin_and_libs ${pkgs.bcachefs.tools}/bin/bcachefs
				copy_bin_and_libs ${pkgs.bcachefs.mount}/bin/mount.bcachefs
			'';
			boot.initrd.extraUtilsCommandsTest = ''
				$out/bin/bcachefs version
				$out/bin/mount.bcachefs --version
			'';
		})
	]);
}
