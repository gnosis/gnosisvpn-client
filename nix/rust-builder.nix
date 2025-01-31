{ crane
, crossSystem ? localSystem
, isCross ? false
, localSystem
, nixpkgs
, rust-overlay
, useRustNightly ? false
} @ args:
let
  crossSystem0 = crossSystem;
in
let
  pkgsLocal = import nixpkgs {
    localSystem = args.localSystem;
    overlays = [ rust-overlay.overlays.default ];
  };

  localSystem = pkgsLocal.lib.systems.elaborate args.localSystem;
  crossSystem =
    let system = pkgsLocal.lib.systems.elaborate crossSystem0; in
    if crossSystem0 == null || pkgsLocal.lib.systems.equals system localSystem
    then localSystem
    else system;

  pkgs = import nixpkgs {
    inherit localSystem crossSystem;
    overlays = [ rust-overlay.overlays.default ];
  };

  # `hostPlatform` is the cross-compilation output platform;
  # `buildPlatform` is the platform we are compiling on
  buildPlatform = pkgs.stdenv.buildPlatform;
  hostPlatform = pkgs.stdenv.hostPlatform;


  envCase = triple: pkgsLocal.lib.strings.toUpper (builtins.replaceStrings [ "-" ] [ "_" ] triple);


  cargoTarget =
    if hostPlatform.config == "armv7l-unknown-linux-gnueabihf" then
      "armv7-unknown-linux-gnueabihf" else hostPlatform.config;

  rustToolchain =
    if useRustNightly
    then pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)
    else
      (pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
        ../rust-toolchain.toml).override { targets = [ cargoTarget ]; };

  craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

  buildEnv = {
    CARGO_BUILD_TARGET = cargoTarget;
    "CARGO_TARGET_${envCase cargoTarget}_LINKER" = "${pkgs.stdenv.cc.targetPrefix}cc";
    HOST_CC = "${pkgs.stdenv.cc.nativePrefix}cc";
  };
in
{
  inherit rustToolchain;

  callPackage = (package: args:
    let crate = pkgs.callPackage package (args // { inherit craneLib isCross; });
    in
    # Override the derivation to add cross-compilation environment variables.
    crate.overrideAttrs (previous: buildEnv // {
      # We also have to override the `cargoArtifacts` derivation with the same changes.
      cargoArtifacts =
        if previous.cargoArtifacts != null
        then previous.cargoArtifacts.overrideAttrs (previous: buildEnv)
        else null;
    }));
}
