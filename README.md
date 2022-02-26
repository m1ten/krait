|                    :warning: WARNING                     |
| :------------------------------------------------------: |
| This project is incomplete and may not work as expected. |

# neo 
[![Workflow Status](https://img.shields.io/github/workflow/status/m1ten/neopkg/compile%20and%20release%20neo%20unstable?logo=github)](https://github.com/m1ten/neopkg/actions/workflows/unstable.yml) [![rustc version](https://img.shields.io/badge/rust-nightly-orange?logo=rust)](https://www.rust-lang.org/) [![Apache-2.0](https://img.shields.io/badge/license-Apache-blue?logo=apache)](./LICENSE)

cross platform package manager

## Installation

Download the latest binary from [releases](https://github.com/m1ten/neopkg/releases)

```sh
# POSIX: Give execution permission to neo and run
$ chmod +x neo && ./neo

# Windows: Run the exe
$ .\neo.exe
```

### Building from source

1. Install dependencies

   1. [Rust Nightly](https://rust-lang.github.io/rustup/concepts/channels.html) using [`rustup`](https://www.rust-lang.org/tools/install)

   ```sh
   $ rustup toolchain install nightly
   ```

2. Clone the [source](https://github.com/m1ten/neopkg) using [`git`](https://git-scm.com/)
   ```sh
   $ git clone https://github.com/m1ten/neopkg.git
   $ cd neo
   ```
   
3. Build and run using [`cargo`](https://doc.rust-lang.org/nightly/cargo/)
   ```sh
   $ cargo +nightly run --release
   ```
