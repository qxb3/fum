{
  config,
  lib,
  fumPackage,
  ...
}: let
  inherit (lib.types) bool package int str;
  inherit (lib.modules) mkIf;
  inherit (lib.options) mkOption mkEnableOption;

  boolToString = x:
    if x
    then "true"
    else "false";
  cfg = config.programs.fum;
  filterOptions = options: builtins.filter (opt: builtins.elemAt opt 1 != "") options;
in {
  options.programs.fum = {
    enable = mkEnableOption "Enable the fum music client.";

    package = mkOption {
      description = "The fum music client package.";
      type = package;
      default = fumPackage;
    };

    players = mkOption {
      description = "List of media players to control.";
      type = lib.types.listOf str;
      default = ["spotify"];
    };

    use_active_player = mkOption {
      description = "Whether to use the active player.";
      type = bool;
      default = true;
    };

    align = mkOption {
      description = "Alignment of the UI.";
      type = str;
      default = "center";
    };

    direction = mkOption {
      description = "Direction of the UI.";
      type = str;
      default = "vertical";
    };

    flex = mkOption {
      description = "Flex alignment of the UI.";
      type = str;
      default = "start";
    };

    width = mkOption {
      description = "Width of the UI.";
      type = int;
      default = 20;
    };

    height = mkOption {
      description = "Height of the UI.";
      type = int;
      default = 18;
    };

    debug = mkOption {
      description = "Enable debug mode.";
      type = bool;
      default = false;
    };

    layout = mkOption {
      description = "Layout configuration.";
      type = lib.types.listOf lib.types.attrs;
      default = [];
    };
  };

  config = mkIf cfg.enable {
    home.packages = [cfg.package];

    xdg.configFile."fum/config.json".text = let
      formatOption = name: value: ''"${name}": ${value}'';
      formatConfig = options:
        builtins.concatStringsSep ",\n" (map (opt:
          formatOption (builtins.head opt)
          (builtins.elemAt opt 1))
        options);
    in ''
      {
        ${formatConfig (filterOptions [
        ["players" (builtins.toJSON cfg.players)]
        ["use_active_player" (boolToString cfg.use_active_player)]
        ["align" (builtins.toJSON cfg.align)]
        ["direction" (builtins.toJSON cfg.direction)]
        ["flex" (builtins.toJSON cfg.flex)]
        ["width" (toString cfg.width)]
        ["height" (toString cfg.height)]
        ["debug" (boolToString cfg.debug)]
        ["layout" (builtins.toJSON cfg.layout)]
      ])}
      }
    '';
  };
}
