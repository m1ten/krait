|                    :warning: WARNING                     |
| :------------------------------------------------------: |
| This project is incomplete and may not work as expected. |

# krait 
[![Workflow Status](https://img.shields.io/github/workflow/status/m1ten/krait/unstable.yml)](https://github.com/m1ten/krait/actions/workflows/unstable.yml) [![rustc version](https://img.shields.io/badge/rust-nightly-orange?logo=rust)](https://www.rust-lang.org/) [![Apache-2.0](https://img.shields.io/badge/license-Apache-blue?logo=apache)](./LICENSE) 

<!-- [![crates.io](https://img.shields.io/crates/v/krait)](https://crates.io/crates/krait) -->

cross platform package manager

## Installation

Download the latest binary from [releases](https://github.com/m1ten/krait/releases)

```sh
# POSIX: Give execution permission to krait and run
$ chmod +x krait && ./krait

# Windows: Run the exe
$ .\krait.exe
```

### Building from source

1. Install dependencies

   1. [Rust Nightly](https://rust-lang.github.io/rustup/concepts/channels.html) using [`rustup`](https://www.rust-lang.org/tools/install)

   ```sh
   $ rustup toolchain install nightly
   ```

2. Clone the [source](https://github.com/m1ten/krait) using [`git`](https://git-scm.com/)
   ```sh
   $ git clone https://github.com/m1ten/krait.git
   $ cd krait
   ```
   
3. Build and run using [`cargo`](https://doc.rust-lang.org/nightly/cargo/)
   ```sh
   $ cargo +nightly run --release
   ```
