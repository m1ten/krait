|                    :warning: WARNING                     |
| :------------------------------------------------------: |
| This project is incomplete and may not work as expected. |

# wix ![](https://github.com/m1ten/wix/workflows/Rust/badge.svg?branch=main) [![rustc version]][rustc]

[rustc version]: https://img.shields.io/badge/rustc-1.58-orange.svg
[rustc]: https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html

wix - cross platform package manager

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

   1. [Rust 1.58](https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html) using [`rustup`](https://www.rust-lang.org/tools/install)

   ```sh
   $ rustup toolchain install stable
   ```

   #### Linux only! (remove `target.x86_64-unknown-linux-gnu` in `.cargo/config.toml` to use gcc)

   2. [Clang 12](https://repology.org/project/llvm/versions)
   3. [mold](https://github.com/rui314/mold)

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
