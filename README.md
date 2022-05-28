|                    :warning: WARNING                     |
| :------------------------------------------------------: |
| This project is incomplete and may not work as expected. |

# wix 
[![Workflow Status](https://img.shields.io/github/workflow/status/m1ten/wix/compile%20and%20release%20neo%20unstable?logo=github)](https://github.com/m1ten/wix/actions/workflows/unstable.yml) [![rustc version](https://img.shields.io/badge/rust-nightly-orange?logo=rust)](https://www.rust-lang.org/) [![crates.io](https://img.shields.io/crates/v/wix)](https://crates.io/crates/wix) [![Apache-2.0](https://img.shields.io/badge/license-Apache-blue?logo=apache)](./LICENSE) 

cross platform package manager

## Installation

Download the latest binary from [releases](https://github.com/m1ten/wix/releases)

```sh
# POSIX: Give execution permission to wix and run
$ chmod +x wix && ./wix

# Windows: Run the exe
$ .\wix.exe
```

### Building from source

1. Install dependencies

   1. [Rust Nightly](https://rust-lang.github.io/rustup/concepts/channels.html) using [`rustup`](https://www.rust-lang.org/tools/install)

   ```sh
   $ rustup toolchain install nightly
   ```

2. Clone the [source](https://github.com/m1ten/wix) using [`git`](https://git-scm.com/)
   ```sh
   $ git clone https://github.com/m1ten/wix.git
   $ cd wix
   ```
   
3. Build and run using [`cargo`](https://doc.rust-lang.org/nightly/cargo/)
   ```sh
   $ cargo +nightly run --release
   ```
