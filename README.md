| :warning: WARNING                                        |
|:--------------------------------------------------------:|
| This project is incomplete and may not work as expected. |

# dash ![](https://github.com/m1ten/dash/workflows/Rust/badge.svg?branch=main) [![Rustc Version]][rustc]

[Rustc Version]: https://img.shields.io/badge/rustc-1.57.0.nightly-lightgray.svg
[rustc]: https://github.com/rust-lang/rust/milestone/86

dash your way through OS post-install

## Installation

Download the latest binary from [releases](https://github.com/m1ten/dash/releases)

```sh
# Unix-Like: Give execution permission to dash and run
$ chmod +x dash && ./dash

# Windows: Run the exe
$ .\dash.exe
```
### Building from source 

1. Install [Rust nightly >=1.57.0](https://github.com/rust-lang/rust/milestone/86) using [`rustup`](https://www.rust-lang.org/tools/install)
   ```sh
   $ rustup toolchain install nightly
   ```
2. Clone the [source](https://github.com/m1ten/dash) using [`git`](https://git-scm.com/)
    ```sh
    $ git clone https://github.com/m1ten/dash.git
    $ cd dash
    ```
3. Build and run using [`cargo`](https://doc.rust-lang.org/stable/cargo/)
    ```sh
    $ cargo +nightly run --release
    ```

## License

dash is licensed under [zlib](./LICENSE).