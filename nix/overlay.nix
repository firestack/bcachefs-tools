{ filter, self, ... }:
final: prev: {
	bcachefs = {
		tools = final.callPackage ../default.nix {
			testWithValgrind = false;
			filter = filter.lib;
			lastModified = builtins.substring 0 8 self.lastModifiedDate;
			versionString = self.version;
		};

		rlibbcachefs = final.callPackage ../rlibbcachefs {};

		mount = final.callPackage ../rlibbcachefs/cmds/mount {};

		sbfind = final.callPackage ../rlibbcachefs/cmds/sbfind {};

		kernelPackages = final.recurseIntoAttrs (final.linuxPackagesFor final.bcachefs.kernel);
		kernel = final.callPackage ./bcachefs-kernel.nix {
			date = "2021-08-05";
			commit = final.bcachefs.tools.bcachefs_revision;
			diffHash = "sha256-viFC3HHIcjUTDPvloSKKsz9PuSLyvxfYnrtkVUB79mQ=";
			kernelPatches = [];
		};
	};
}
