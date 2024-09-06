{
  pkgs,
  lib,
  rustToolchain,
  rootDir,
  ...
}: let
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustToolchain;
    rustc = rustToolchain;
  };

  cargoFile = /. + rootDir + "/fuzon/Cargo.toml";
  lockFile = /. + rootDir + "/fuzon/Cargo.lock";
in
  rustPlatform.buildRustPackage {
    name = "fuzon";
    src = /. + rootDir;

    version = (lib.importTOML cargoFile).package.version;

    cargoLock = {
      inherit lockFile;
    };

    meta = {
      description = "A CLI tool to fuzzy search ontology terms by their labels.";
      homepage = "https://github.com/sdsc-ordes/fuzon";
      license = lib.licenses.asl20;
      maintainers = ["gabyx" "cmdoret"];
    };
  }
