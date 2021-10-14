{
	description = "Userspace tools for bcachefs";

	# Nixpkgs / NixOS version to use.
	inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
	inputs.utils.url = "github:numtide/flake-utils";
	inputs.filter.url = "github:numtide/nix-filter";

	outputs = { self, nixpkgs, utils, filter, ... }@inputs:
		let
			# System types to support.
			supportedSystems = [ "x86_64-linux" ];
		in
		{
			version = "${builtins.substring 0 8 self.lastModifiedDate}-${self.shortRev or "dirty"}";

			overlay = import ./nix/overlay.nix inputs;
		}
		// utils.lib.eachSystem supportedSystems (system: 
		let pkgs = import nixpkgs { 
			inherit system; 
			overlays = [ self.overlay ]; 
		}; 
		in rec {
			
			# A Nixpkgs overlay.

			# Provide some binary packages for selected system types.
			defaultPackage = pkgs.bcachefs.tools;
			packages = {
				inherit (pkgs.bcachefs)
					mount
					tools
					toolsValgrind
					toolsDebug
					bch_bindgen
					kernel;

				musl-tools = pkgs.pkgsMusl.bcachefs.tools;
				musl-mount = pkgs.pkgsMusl.bcachefs.mount;
			};

			checks = { 
				kernelSrc = packages.kernel.src;
				inherit (packages) 
					toolsValgrind;

				# Build and test initrd with bcachefs and bcachefs.mount installed
				bootStage1Module = (nixpkgs.lib.nixosSystem { 
					inherit system; 
					modules = [ 
						("${nixpkgs}/nixos/modules/installer/netboot/netboot.nix")
						self.nixosModule 
						self.nixosModules.bcachefs-enable-boot
					]; 
				}).config.system.build.bootStage1;
			};

			devShell = devShells.tools;
			devShells.tools = pkgs.bcachefs.tools.override { inShell = true; };
			devShells.mount = pkgs.bcachefs.mount.override { inShell = true; };
		});
}
