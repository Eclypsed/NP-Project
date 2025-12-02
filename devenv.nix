{
  pkgs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    raylib
    marp-cli
  ];

  # https://devenv.sh/languages/
  languages = {
    c.enable = true;
    python = {
      enable = true;
      venv = {
        enable = true;
        requirements = ''
          click==8.3.1
        '';
      };
    };
    rust.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
