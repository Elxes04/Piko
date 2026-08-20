{
  description = "A fast, lightweight system information tool written in Rust, inspired by Neofetch";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system:
        let pkgs = nixpkgs.legacyPackages.${system};
        in f { inherit system pkgs; });
    in
    {
      packages = forAllSystems ({ system, pkgs }: rec {
        piko = pkgs.rustPlatform.buildRustPackage {
          pname = "piko";
          version = "1.2.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = with pkgs.lib; {
            description = "A minimal, fast, and customizable system information tool in Rust";
            homepage = "https://github.com/Elxes04/Piko";
            license = licenses.mit;
            mainProgram = "piko";
          };
        };
        default = piko;
      });

      apps = forAllSystems ({ system, pkgs }: rec {
        piko = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/piko";
        };
        default = piko;
      });

      devShells = forAllSystems ({ system, pkgs }: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            clippy
            rustfmt
          ];
        };
      });

      overlays.default = final: prev: {
        piko = self.packages.${prev.stdenv.hostPlatform.system}.default;
      };
    };
}
