{
  pkgs,
  rootSrc,
  lib,
  makeRustPlatform,
  rustToolchain,
  python313,
}: let
  rustPlatform = makeRustPlatform {
    cargo = rustToolchain;
    rustc = rustToolchain;
  };

  fs = lib.fileset;
  cargoFile = "${rootSrc}/src/fuzon/Cargo.toml";
  lockFile = "${rootSrc}/Cargo.lock";
in
  (rustPlatform.buildRustPackage
    {
      name = "fuzon";

      src = fs.toSource {
        root = rootSrc;
        fileset = fs.gitTracked rootSrc;
      };

      version = (lib.importTOML cargoFile).package.version;

      cargoLock = {
        inherit lockFile;
      };

      dontInstallDocs = true;
      dontInstallManpages = true;
      doInstallCargoArtifacts = false;


      meta = {
        description = "A CLI tool to fuzzy search ontology terms by their labels.";
        homepage = "https://github.com/sdsc-ordes/fuzon";
        license = lib.licenses.asl20;
        maintainers = ["gabyx" "cmdoret"];
      };
    })
  .overrideAttrs (finalAttrs: prevAttrs: {
    doCheck = false; # disable cargo test (requires network)
    nativeBuildInputs = prevAttrs.nativeBuildInputs ++ [python313 pkgs.perl];
  })
