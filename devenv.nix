{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  languages.rust.enable = true;

  git-hooks.hooks = {
    alejandra.enable = true;
    cargo-check.enable = true;
    clippy.enable = true;
    rustfmt.enable = true;
  };
}
