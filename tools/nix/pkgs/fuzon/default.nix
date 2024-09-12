{
  self,
  lib,
  makeRustPlatform,
  rustToolchain,
}: let
  rustPlatform = makeRustPlatform {
    cargo = rustToolchain;
    rustc = rustToolchain;
  };

  fs = lib.fileset;
  rootDir = ../../../..;

  cargoFile = "${rootDir}/fuzon/Cargo.toml";
  lockFile = "${rootDir}/Cargo.lock";
in
  rustPlatform.buildRustPackage {
    name = "fuzon";

    src = fs.toSource {
      root = rootDir;
      fileset = fs.gitTracked rootDir;
    };

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
