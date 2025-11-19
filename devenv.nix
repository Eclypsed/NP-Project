{
  pkgs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    raylib
  ];

  # https://devenv.sh/languages/
  languages = {
    c.enable = true;
    python = {
      enable = true;
      venv = {
        enable = true;
      };
    };
    rust.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
