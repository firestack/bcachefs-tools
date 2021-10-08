{ filter, self, ... }:
final: prev: {
	bcachefs = {
		tools = final.callPackage ../default.nix {
			testWithValgrind = false;
			filter = filter.lib;
			lastModified = builtins.substring 0 8 self.lastModifiedDate;
			versionString = self.version;
		};

		mount = final.callPackage ../mount {
		};

		kernel = final.callPackage ./bcachefs-kernel.nix {
			date = "2021-08-05";
			commit = final.bcachefs.tools.bcachefs_revision;
			diffHash = "sha256-9NUTmC8FnXJzJ0tF2FrGW10fuGSRVq3ONdSzVmoOSTs=";
			kernelPatches = [];
		};
		
		rlibbcachefs = final.callPackage ../rlibbcachefs {
		};

		sbfind = final.callPackage ../cmds/sbfind {};
		
		kernelPackages = final.recurseIntoAttrs (final.linuxPackagesFor final.bcachefs.kernel);
	};
}
