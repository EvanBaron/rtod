{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    (python3.withPackages (
      ps: with ps; [
        pip
      ]
    ))
    libGL
    glib
    stdenv.cc.cc.lib
    zlib
    libxcb
    libX11
    libXext
    libXrender
    libICE
    libSM
    libxkbcommon
  ];

  shellHook = ''
    if [ ! -d .venv ]; then
      echo "Creating virtual environment..."
      python -m venv .venv
    fi

    source .venv/bin/activate

    export LD_LIBRARY_PATH=${
      pkgs.lib.makeLibraryPath [
        pkgs.libGL
        pkgs.glib
        pkgs.stdenv.cc.cc.lib
        pkgs.zlib
        pkgs.libxcb
        pkgs.libX11
        pkgs.libXext
        pkgs.libXrender
        pkgs.libICE
        pkgs.libSM
        pkgs.libxkbcommon
      ]
    }:$LD_LIBRARY_PATH
    export QT_QPA_PLATFORM=xcb
    export PIP_USER=0

    # Reinstall only when prototype/requirements.txt changes
    stamp=.venv/.requirements.sha256
    if [ -f prototype/requirements.txt ] && ! sha256sum -c "$stamp" --status 2>/dev/null; then
      echo "Installing Python dependencies (first run downloads PyTorch, may take a while)..."
      pip install --upgrade pip && pip install -r prototype/requirements.txt \
        && sha256sum prototype/requirements.txt > "$stamp"
    fi

    echo "Environment ready!"
  '';
}
