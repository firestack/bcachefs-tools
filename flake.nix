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
			nixosModule = import ./mount/module.nix;
		}
		// utils.lib.eachSystem supportedSystems (system: 
		let pkgs = import nixpkgs { 
			inherit system; 
			overlays = [

				self.overlay

		]; }; in rec {
			
			# A Nixpkgs overlay.

			# Provide some binary packages for selected system types.
			packages = {
				inherit (pkgs.bcachefs)
					mount
					tools
					rlibbcachefs
					sbfind
					kernel;

				musl-tools = pkgs.pkgsMusl.bcachefs.tools;
				musl-mount = pkgs.pkgsMusl.bcachefs.mount;
			};

			devShells.tools = pkgs.bcachefs.tools.override { inShell = true; };
			devShells.mount = pkgs.bcachefs.mount.override { inShell = true; };
			devShell = devShells.tools;

			defaultPackage = pkgs.bcachefs.tools;
		});
			

}
