#!/home/ubuntu/bin/rust-scripts/using_rust_with_python/venv/bin/python3

"""
PyCalc CLI using Python Fire

Call the Python methods in Calculator from the CLI:
./calc.py add 2 2
./calc.py subtract 5 2
./calc.py multiply 3 4
./calc.py divide 42 2
"""

import fire

from libpycalc_cli import (
    add_as_string,
    subtract_as_string,
    multiply_as_string,
    divide_as_string,
)


class Calculator(object):
    """Rust Calculator Class"""
    
    def add(self, num1, num2):
        """Add two numbers"""
        return add_as_string(num1, num2)
    
    def subtract(self, num1, num2):
        """Subtract two numbers"""
        return subtract_as_string(num1, num2)
        
    def multiply(self, num1, num2):
        """Multiply two numbers"""
        return multiply_as_string(num1, num2)
        
    def divide(self, num1, num2):
        """Divide two numbers"""
        return divide_as_string(num1, num2)
        

if __name__ == "__main__":
    fire.Fire(Calculator)
