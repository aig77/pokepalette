{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default";
    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, systems, ... } @ inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (system: {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
        devenv-test = self.devShells.${system}.default.config.test;
      });

      devShells = forEachSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            default = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                {
                  packages = with pkgs; [
                    openssl
                    pkg-config
                  ];

                  languages = {
                    rust = {
                      enable = true;
                      channel = "stable"; # nixpkgs, stable, beta, nightly
                      components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
                      version = "latest";
                    };
                  };

                  git-hooks.hooks = {
                    rustfmt.enable = true;
                    prettier.enable = true;
                  };

                  env = {
                    RUST_BACKTRACE = 1;
                    OPENSSL_DIR = "${pkgs.openssl.dev}";
                    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
                  };

                  enterShell = ''
                    echo ""
                    echo "⚡️ Entered Pokepalette dev shell! 🦀"
                  '';
                }
              ];
            };
          });
    };
}
