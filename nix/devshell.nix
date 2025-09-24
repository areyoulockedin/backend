{pkgs}:
with pkgs;
  mkShell {
    buildInputs = with pkgs; [
      rustup
      postgresql
    ];

    shellHook = ''
      echo "Welcome to the AreYouLocked.in development shell!"
    '';
  }
