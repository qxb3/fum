{
  nixosModules = import ./modules/default.nix;
  homeManagerModules = import ./hm-module.nix;
}
