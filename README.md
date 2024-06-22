# hovertank-game
A spiritual successor to Wulfram 2, written in Rust with Bevy and open source because hell yeah

## Building
### Dependencies
For building on Windows, ensure you have `rust-lld.exe` by running the following:  
```shell
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

For building on Linux, ensure you have [`mold`](https://github.com/rui314/mold) by running:
- Ubuntu:
    ```shell
        sudo apt-get install mold clang
    ```
- Fedora: 
    ```
        sudo dnf install mold clang
    ```
- Arch:
    ```
        sudo pacman -S mold clang
    ```

Other platforms have no specific build dependencies.

### Building the source code
After ensuring your system has the necessary dependencies, run:
```shell
cargo run
```
to run a debug version of the game.  To make a release build, run:
```shell
cargo run --release
```
