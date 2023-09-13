let
  my-python-packages = p:
    with p; [
      # pandas
      requests
      pip
      APScheduler
      screeninfo
      psutil
      # other python packages
    ];
in
  {pkgs ? import <nixpkgs> {}}:
    pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        gnumake
        xorg.libX11.dev
        xorg.libXcursor.dev
        xorg.libXft
        xorg.libXinerama
        (python3.withPackages my-python-packages)
        alejandra
      ];
    }
