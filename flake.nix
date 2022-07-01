{
	description = "Build a cargo project";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

		crane = {
			url = "github:ipetkov/crane";
			inputs.nixpkgs.follows = "nixpkgs";
		};

		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, crane, flake-utils, ... }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs {
					inherit system;
				};

				inherit (pkgs) lib;

				craneLib = crane.lib.${system};
				src = ./.;

				# Build *just* the cargo dependencies, so we can reuse
				# all of that work (e.g. via cachix) when running in CI
				cargoArtifacts = craneLib.buildDepsOnly {
					inherit src;
				};

				# Build the actual crate itself, reusing the dependency
				# artifacts from above.
				cdp-print = craneLib.buildPackage {
					inherit cargoArtifacts src;
				};
			in
			{
				checks = {
					# Build the crate as part of `nix flake check` for convenience
					inherit cdp-print;

					# Run clippy (and deny all warnings) on the crate source,
					# again, resuing the dependency artifacts from above.
					#
					# Note that this is done as a separate derivation so that
					# we can block the CI if there are issues here, but not
					# prevent downstream consumers from building our crate by itself.
					cdp-print-clippy = craneLib.cargoClippy {
						inherit cargoArtifacts src;
						cargoClippyExtraArgs = "-- --deny warnings";
					};

					# Check formatting
					cdp-print-fmt = craneLib.cargoFmt {
						inherit src;
					};
				} // lib.optionalAttrs (system == "x86_64-linux") {
					# NB: cargo-tarpaulin only supports x86_64 systems
					# Check code coverage (note: this will not upload coverage anywhere)
					cdp-print-coverage = craneLib.cargoTarpaulin {
						inherit cargoArtifacts src;
					};
				};

				packages.default = cdp-print;

				apps.default = flake-utils.lib.mkApp {
					drv = cdp-print;
				};

				devShells.default = pkgs.mkShell {
					inputsFrom = builtins.attrValues self.checks;

					# Extra inputs can be added here
					nativeBuildInputs = with pkgs; [
						cargo
						rustc
					];
				};
			});
}
