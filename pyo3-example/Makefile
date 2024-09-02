init:
	maturin init

build:
	cargo build --release
	cp target/release/pyo3_example.so .

clean:
	# Remove the build .so file
	rm -f pyo3_example.so

