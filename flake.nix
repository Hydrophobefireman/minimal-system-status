{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        
      ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          nonRustDeps = [
            pkgs.libiconv
          ];
          rust-toolchain = pkgs.symlinkJoin {
            name = "rust-toolchain";
            paths = [ pkgs.rustc pkgs.cargo pkgs.cargo-watch pkgs.rust-analyzer pkgs.rustPlatform.rustcSrc ];
          };
          bin = pkgs.rustPlatform.buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
          dockerImage =  pkgs.dockerTools.buildImage {
            name = "minimal-system-status";
            tag = "latest";
            copyToRoot = with pkgs.dockerTools; [
                  bin
                  # binSh
                  # usrBinEnv
                  # bin
                  # pkgs.coreutils
                  # pkgs.wget
                  # caCertificates
                  # fakeNss
            ];
            config = {
              Cmd = [ "/bin/minimal-system-status" ];
            };
        };
        in
        {
          
          # Rust package
          packages = {
            inherit bin dockerImage;
            default = bin;
          };

          # Rust dev environment
          devShells.default = pkgs.mkShell {
            inputsFrom = [
              
            ];
            shellHook = ''
              # For rust-analyzer 'hover' tooltips to work.
              export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
              echo
              echo "Run 'just <recipe>' to get started"
              just
            '';
            buildInputs = nonRustDeps;
            nativeBuildInputs = with pkgs; [
              just
              rust-toolchain
            ];
            RUST_BACKTRACE = 1;
          };

        
        };
    };
}
