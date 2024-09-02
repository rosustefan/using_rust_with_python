#!/home/ubuntu/bin/rust-scripts/using_rust_with_python/venv/bin/python3

import pyo3_example


pi_sum_as_string = pyo3_example.sum_as_string(21, 42)
print(pi_sum_as_string)

