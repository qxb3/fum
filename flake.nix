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

      updateScript = pkgs.writeShellScriptBin "update-fum" ''
        nix flake update

        ${pkgs.nix-update}/bin/nix-update -vr 'v(.*)' --flake --commit fum
      '';
    in {
      packages.fum = pkgs.rustPlatform.buildRustPackage rec {
        pname = "fum";
        version = "0.8.2";

        src = pkgs.fetchFromGitHub {
          owner = "qxb3";
          repo = pname;
          rev = "v${version}";
          hash = "sha256-KOxT7h7HcI3AsWKTV7BjJeVCkzReMHu3Xl6oGD+JjJw=";
        };

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
          autoPatchelfHook
        ];

        buildInputs = with pkgs; [
          openssl
          dbus
          libgcc
        ];

        OPENSSL_DIR = "${pkgs.openssl.dev}";
        OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";

        meta = with pkgs.lib; {
          description = "A fully ricable tui-based music client";
          homepage = "https://github.com/qxb3/fum";
          license = licenses.mit;
          maintainers = with maintainers; [linuxmobile];
          platforms = platforms.linux;
        };
      };

      packages.default = self.packages.${system}.fum;

      devShells.default = pkgs.mkShell {
        inputsFrom = [self.packages.${system}.fum];
        buildInputs = with pkgs; [
          cargo
          rust-analyzer
          rustfmt
          clippy
          updateScript
        ];
      };

      apps.update = {
        type = "app";
        program = "${updateScript}/bin/update-fum";
      };

      nixosModules.fum = import ./modules/default.nix;

      homeManagerModules.fum = {
        config,
        pkgs,
        lib,
        ...
      }:
        import ./nix/hm-module.nix {
          inherit config pkgs lib;
          fumPackage = self.packages.${system}.fum;
        };
    });
}
