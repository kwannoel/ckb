# Environment for building ckb
# cargo install --path . -f --locked
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
    libsodium
  ];
     # export SNAPPY_LIB_DIR=/usr/local/lib
  shellHook = ''
     export LIBCLANG_PATH=${pkgs.llvmPackages.libclang.lib}/lib
     alias capsule=$HOME/.cargo/bin/capsule
     alias ckb-cli=$HOME/projects/ckb-cli/target/release/ckb-cli
     alias ckb=$HOME/projects/ckb/target/release/ckb
     function cdp {
       cd ~/projects/ckb
     }
     function cck {
       cargo check
     }
     function mkbin {
       cargo install --path . -f --locked
       cp ~/.cargo/bin/ckb ~/.local/bin/ckb
     }
  '';
}
