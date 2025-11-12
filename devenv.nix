{
  pkgs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = with pkgs; [ git ];

  # https://devenv.sh/languages/
  languages.python = {
    enable = true;
    venv = {
      enable = true;
    };
  };

  # See full reference at https://devenv.sh/reference/options/
}
