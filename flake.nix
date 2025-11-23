{
  inputs.nixpkgs.url = "https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz";

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      system = "x86_64-linux";

      pkgs = import nixpkgs {
        system = "x86_64-linux";
      };
    in
    {
      packages.${system}.default = pkgs.callPackage ./. { };

      devShells.${system}.default = pkgs.mkShell {
        inputsFrom = [ self.packages.${system}.default ];
        packages = [
          pkgs.rust-analyzer
        ];
      };
    };
}
