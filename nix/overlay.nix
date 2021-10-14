{ filter, self, ... }:
final: prev: {
	bcachefs = {
		tools = final.callPackage ../default.nix {
			testWithValgrind = false;
			filter = filter.lib;
			lastModified = builtins.substring 0 8 self.lastModifiedDate;
			versionString = self.version;
		};
		toolsValgrind = final.bcachefs.tools.override {
			testWithValgrind = true;
		};
		toolsDebug = final.bcachefs.toolsValgrind.override {
			debugMode = true;
		};

		bch_bindgen = final.callPackage ../rust-src/bch_bindgen {};

		mount = final.callPackage ../rust-src/mount {};

		rlibbcachefs = final.callPackage ../rust-src/rlibbcachefs {};
		
		sbfind = final.callPackage ../rust-src/sbfind {};

		kernelPackages = final.recurseIntoAttrs (final.linuxPackagesFor final.bcachefs.kernel);
		kernel = final.callPackage ./bcachefs-kernel.nix {
			commit = final.bcachefs.tools.bcachefs_revision;
			# This needs to be recalculated for every revision change
			diffHash = "sha256-JrVRkEO7DKUTf+qhjWPwfbF3a/Qbd8me7oGay4aae3k=";
			kernelPatches = [];
		};
	};
}
