[![Rust](https://github.com/codeonwort/pvr/actions/workflows/rust.yml/badge.svg)](https://github.com/codeonwort/pvr/actions/workflows/rust.yml)

# Overview
Study 'Production Volume Rendering' and implement it with Rust.

- Reference : [Production Volume Rendering](https://github.com/pvrbook/pvr) by Magnus Wrenninge
- Language  : [Rust](https://www.rust-lang.org/)

# Project Structure
- pvrlib : Volume rendering library
- app    : Test GUI application

# Third-party Libraries
- [rayon 1.1](https://docs.rs/rayon/1.1.0/rayon/index.html) for multithreading
- [bit-vec 0.6](https://crates.io/crates/bit-vec/0.6.3) for bit vector
- [image 0.23.9](https://docs.rs/image/0.23.9/image/index.html) for image IO
- [druid 0.6.0](https://docs.rs/druid/0.6.0/druid/index.html) for GUI
- [native-dialog 0.5.5](https://docs.rs/native-dialog/0.5.5/native_dialog/index.html) for GUI
- [sys-info 0.9.1](https://docs.rs/sys-info/0.9.1/sys_info/index.html) for CPU info query

# Sample Images
--> Embed fancy images here <--

![showcase0](https://user-images.githubusercontent.com/11644393/212523581-ed61676c-8977-4a60-8aab-c7241de9bc99.jpg)
![showcase1](https://user-images.githubusercontent.com/11644393/212523590-384da337-e932-4039-9589-8f84f53b08e3.jpg)

Not so fancy image