{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "fetch-cargo-index";
  version = "none";

  cargoLock.lockFile = ./Cargo.lock;
})
