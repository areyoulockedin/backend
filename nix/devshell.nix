{
  pkgs,
  rust-overlay,
}:
with pkgs;
  mkShell {
    buildInputs = [
      rust-bin.stable.latest.default.override
      {
        extensions = ["rustfmt" "clippy" "rustanalyzer"];
      }
      postgresql
    ];

    shellHook = ''
      echo "Welcome to the AreYouLocked.in development shell!"
    '';
  }
