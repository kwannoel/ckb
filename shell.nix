{ pkgs ? import <nixpkgs> {} }: # TODO: This is nixpkgs 21.05, pin the nixpkgs ver

with pkgs;

mkShell {
  buildInputs = [
    git
    gcc
    # libc6-dev?
    pkg-config
    # libssl-dev?
    # libclang-dev
    llvm
    openssl
    clang
    llvmPackages.libclang
    rustc
  ];
     # export SNAPPY_LIB_DIR=/usr/local/lib
  shellHook = ''
     LIBCLANG_PATH=${pkgs.llvmPackages.libclang.lib}/lib
  '';
}
