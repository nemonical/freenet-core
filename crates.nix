{...}: {
  perSystem = {
    pkgs,
    config,
    ...
  }: let
    # TODO: change this to your crate's name
    crateName = "freenet-core";
    commonDeps = {
      buildInputs = with pkgs; [ openssl.dev ];
      nativeBuildInputs = with pkgs; [ pkg-config ];
    };
  in {
    # declare projects
    # TODO: change this to your crate's path
    nci.projects.${crateName} = {
      path = ./.;
      depsDrvConfig.mkDerivation = commonDeps;
      drvConfig.mkDerivation = commonDeps;
      targets = {
        x86_64-unknown-linux-gnu.default = true;
	wasm32-unknown-unknown.profiles = [ "all profiles" ];
      };
    };
    # configure crates
    nci.crates.${crateName} = {};
  };
}
