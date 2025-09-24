{pkgs}:
with pkgs; let
  rustToolchain = rust-bin.stable.latest.default.override {
    extensions = ["rustfmt" "clippy" "rust-analyzer"];
  };
in
  mkShell {
    buildInputs = with pkgs; [
      rustToolchain
      postgresql
    ];

    shellHook = ''
      echo "Welcome to the AreYouLocked.in development shell!"
    '';
  }
