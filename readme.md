# [Using Python with Rust](https://www.linkedin.com/learning/using-rust-with-python/pyo3-installation?u=2165786)

## Useful commands
- `maturin init`
The maturin init command is used to initialize a new Python project written in Rust. This command sets up the required files and directory structure to create a Python extension module in Rust using the pyo3 crate (e.g., `Cargo.toml` and `lib.rs` are created).

- `maturin develop`
The maturin develop command is used to build and install your Rust-based Python extension in development mode. This command compiles the Rust code into a shared library (a .so file on Linux/macOS or .pyd on Windows) and installs it into the current Python virtual environment or global Python installation for testing.

- `maturin build`
This command is used when you want to create distributable Python packages. It compiles your Rust code and packages it as a Python wheel (.whl file) or source distribution (.tar.gz).

- `make build` (run this as the final step)
Create and edit the Makefile && run `make build`