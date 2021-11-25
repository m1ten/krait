## Installation

Download the latest binary from [releases](https://github.com/m1ten/wix/releases)

```sh
# Unix-Like: Give execution permission to wix and run
$ chmod +x wix && ./wix

# Windows: Run the exe
$ .\wix.exe
```
### Building from source 

1. Install dependencies
   1. [Python >=3.8](https://python.org/)
   2. [Rust >=1.56.0](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html) using [`rustup`](https://www.rust-lang.org/tools/install)
   ```sh
   $ rustup toolchain install stable
   ```

2. Clone the [source](https://github.com/m1ten/wix) using [`git`](https://git-scm.com/)
    ```sh
    $ git clone https://github.com/m1ten/wix.git
    $ cd wix
    ```
3. Build and run using [`cargo`](https://doc.rust-lang.org/stable/cargo/)
    ```sh
    $ cargo +stable run --release
    ```

## License

wix is licensed under [zlib](./LICENSE).