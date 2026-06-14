{
  description = "choomsay — a cyberpunk choom that speaks your text in a terminal bubble";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        # Reproducible build from source. buildRustPackage runs `cargo test`
        # in its checkPhase, so `nix build` / `nix flake check` also test.
        choomsay = pkgs.rustPlatform.buildRustPackage {
          pname = "choomsay";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock; # zero deps → no vendor hash needed

          meta = with pkgs.lib; {
            description = "a cyberpunk choom that speaks your text in a terminal bubble";
            homepage = "https://github.com/vibechoom/choomsay";
            license = licenses.mit;
            mainProgram = "choomsay";
          };
        };
      in
      {
        packages.default = choomsay;
        packages.choomsay = choomsay;

        # `nix run` → the choom speaks
        apps.default = {
          type = "app";
          program = "${choomsay}/bin/choomsay";
        };

        # `nix flake check` → build + run the test suite, reproducibly
        checks.default = choomsay;

        # `nix develop` → identical toolchain for everyone
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
          ];
        };
      }
    );
}
