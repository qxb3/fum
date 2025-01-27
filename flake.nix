{
  description = "Fum: A fully ricable tui-based music client";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      rustPlatform = pkgs.rustPlatform;
      envVars = ''
        export OPENSSL_DIR=${pkgs.openssl.dev}
        export OPENSSL_LIB_DIR=${pkgs.openssl.out}/lib
        export OPENSSL_INCLUDE_DIR=${pkgs.openssl.dev}/include
        export DEP_OPENSSL_INCLUDE=${pkgs.openssl.dev}/include
        export PKG_CONFIG_ALLOW_CROSS=1
        export PKG_CONFIG_PATH=${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.dbus.dev}/lib/pkgconfig
        export CARGO_TARGET_DIR=target
      '';
      fumPackage = rustPlatform.buildRustPackage {
        pname = "fum";
        version = "v0.4.1";

        src = ./.;

        cargoHash = "sha256-q6QfIlePhTcruBIH/Qr2F5H5HzfsGQWjsyaQNpZdVc8=";

        nativeBuildInputs = with pkgs; [
          openssl
          pkg-config
          dbus
        ];

        buildInputs = with pkgs; [
          openssl
          dbus
        ];

        buildPhase = ''
          ${envVars}
          cargo build --release
        '';

        installPhase = ''
          mkdir -p $out/bin
          cp target/release/fum $out/bin
        '';

        meta = with pkgs.lib; {
          description = "A fully ricable tui-based music client";
          license = licenses.mit;
          maintainers = ["qxbt" "linuxmobile"];
          platforms = platforms.linux;
        };
      };
    in {
      defaultPackage = fumPackage;
      packages = {
        fum = fumPackage;
      };

      devShells = {
        default = pkgs.mkShell {
          buildInputs = [pkgs.cargo];
          shellHook = ''
            ${envVars}
          '';
        };
      };

      nixosModules = {
        fum = import ./modules/default.nix;
      };

      homeManagerModules = {
        fum = {
          config,
          pkgs,
          lib,
          ...
        }:
          import ./nix/hm-module.nix {inherit config pkgs lib fumPackage;};
      };
    });
}
