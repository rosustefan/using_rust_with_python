#!/home/ubuntu/bin/rust-scripts/using_rust_with_python/venv/bin/python3

import libownership_pyrust


list_instance = libownership_pyrust.NumberList()

print("Inserting two numbers: 21, and 42")
list_instance.add(21)
list_instance.add(42)

print("Printing the list")
print(list_instance.length())  # Outputs: 2

print("Clearing the list")
list_instance.clear_list()
print("Printing the cleard list which should be empty")
print(list_instance.length())  # Outputs: 0
