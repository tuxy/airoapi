{
  pkgs,
  ...
}:
{
  packages = with pkgs; [
    wrangler
    pkg-config
    openssl
  ];

  # https://devenv.sh/languages/
  languages = {
    rust = {
      enable = true;
      lsp.enable = true;
      targets = [ "wasm32-unknown-unknown" ];
      channel = "stable";
    };
    javascript = {
      enable = true;
      npm.enable = true;
    };
  };

  # See full reference at https://devenv.sh/reference/options/
}
