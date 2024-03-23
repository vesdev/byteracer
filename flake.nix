{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, flake-parts, rust-overlay, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" ];

      perSystem = { pkgs, system, ... }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl
            rustPlatform.bindgenHook
          ];

          buildInputs = with pkgs; [ ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

          devPackages = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = [
                "cargo"
                "clippy"
                "rust-src"
                "rust-analyzer"
                "rustc"
                "rustfmt"
              ];
            })
            diesel-cli
          ];
        in {
          devShells = {
            default = pkgs.mkShell {
              inherit LD_LIBRARY_PATH buildInputs nativeBuildInputs;
              packages = devPackages;
            };
          };
        };
    };
}
