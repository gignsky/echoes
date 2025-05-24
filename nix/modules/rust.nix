{ inputs, ... }:
{
  imports = [
    inputs.rust-flake.flakeModules.default
    inputs.rust-flake.flakeModules.nixpkgs
    inputs.process-compose-flake.flakeModule
    inputs.cargo-doc-live.flakeModule
  ];
  perSystem = { config, self', pkgs, lib, ... }: {
    rust-project.crates."echoes".crane.args = {
      buildInputs = lib.optionals pkgs.stdenv.isDarwin (
        with pkgs.darwin.apple_sdk.frameworks; [
          IOKit
        ]
      );

      postInstall = ''
        echo "--- Contents of current directory AFTER installPhase ---"
        ls -lA .
        echo "--- Contents of target/release/ AFTER installPhase ---"
        ls -lA target/release/
        
        echo "--- Copying binary from target/release/ to $out/bin/ ---"
        mkdir -p $out/bin
        cp target/release/echoes $out/bin/

        echo "--- Contents of $out/bin/ after installPhase ---"
        ls -lA $out/bin/
        test -x $out/bin/echoes && echo "echoes is EXECUTABLE in $out/bin (postInstall check)" || echo "echoes is NOT EXECUTABLE in $out/bin (postInstall check)"
      
        wrapProgram $out/bin/echoes \
          --set LOLCAT_PATH ${pkgs.lolcat}/bin/lolcat
      '';
    };
    packages.default = self'.packages.echoes;
  };
}
