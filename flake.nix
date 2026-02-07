{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        with pkgs;
        {
          devShells.default = mkShell {
            buildInputs = [
              rust-bin.stable.latest.default
              awscli2
              zola
              python312Packages.botocore
            ];
            RUST_SRC_PATH="${pkgs.latest.rustChannels.stable.rust-src}/lib/rustlib/src/rust/library/";
          };

          packages = {
            dev-local = pkgs.writeShellScriptBin "dev-local" ''
              echo "Copying markdown files to static..."
              ${pkgs.bash}/bin/bash ./build.sh
              echo "Starting local development server..."
              ${pkgs.zola}/bin/zola serve
            '';

            dev-lan = pkgs.writeShellScriptBin "dev-lan" ''
              echo "Copying markdown files to static..."
              ${pkgs.bash}/bin/bash ./build.sh
              echo "Starting LAN development server on 0.0.0.0..."
              ${pkgs.zola}/bin/zola serve --interface 0.0.0.0
            '';
          };

          apps = {
            dev-local = {
              type = "app";
              program = "${self.packages.${system}.dev-local}/bin/dev-local";
            };

            dev-lan = {
              type = "app";
              program = "${self.packages.${system}.dev-lan}/bin/dev-lan";
            };
          };
        }
  );
}