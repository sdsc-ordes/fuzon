{
  pkgs,
  fuzon,
}:
pkgs.dockerTools.buildLayeredImage {
  name = "ghcr.io/sdsc-ordes/fuzon";
  tag = fuzon.version;

  contents = [fuzon];

  fakeRootCommands = ''
    ${pkgs.dockerTools.shadowSetup}
    groupadd -r non-root
    useradd -r -g non-root non-root
    mkdir -p /workspace
    chown non-root:non-root /workspace
  '';
  enableFakechroot = true;

  config = {
    Entrypoint = ["fuzon"];
    WorkingDir = "/workspace";
    Labels = {
      "org.opencontainers.image.source" = "https://github.com/sdsc-ordes/fuzon";
      "org.opencontainers.image.description" = fuzon.meta.description;
      "org.opencontainers.image.license" = "Apache-2.0";
    };
    User = "non-root";
  };
}
