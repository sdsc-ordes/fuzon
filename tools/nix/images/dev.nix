{ pkgs, devShellDrv, ... }:

let
  image_name = "ghcr.io/sdsc-ordes/fuzon";

in pkgs.dockerTools.buildNixShellImage {
      name = image_name;
      tag = "dev";
      drv = devShellDrv;
}
