# vitagl-sys

Low-level Rust bindings for the vitaGL library on PlayStation Vita.

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/Exotik850/vitagl-sys)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-manual-lightgrey.svg)](https://github.com/Exotik850/vitagl-sys)

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)
- [Support](#support)

## Installation

This crate targets the PlayStation Vita toolchain and is intended as a dependency for higher-level wrappers. You will need the VitaSDK and the native prerequisites required by `bindgen`.

Prerequisites:
- latest Rust nightly toolchain
- VitaSDK installed and the `VITASDK` environment variable set
- `libclang` and C headers available to `bindgen`

Add the dependency:

```toml
[dependencies]
vitagl-sys = "0.1.0"
```

## Usage

The API mirrors the C headers closely and is `unsafe` by design. Use it directly only when you need raw FFI access.

```rust
use vitagl_sys::*;

fn main() {
	unsafe {
		vglInit(0x800000);
		glClearColor(0.0, 0.0, 0.0, 0.0);
		glClear(GL_COLOR_BUFFER_BIT);
		vglSwapBuffers(GL_FALSE as u8);
	}
}
```

If you have `cargo-vita` installed, you can build the example VPK:

```bash
cargo vita build vpk -- --release --example cube
```

For documentation, you can generate it locally: 

```bash
cargo doc --open --target armv7-sony-vita-newlibeabihf
```

## License

MIT. See [LICENSE](LICENSE).

## Support

Open an issue on GitHub for questions or bug reports: <https://github.com/Exotik850/vitagl-sys/issues>