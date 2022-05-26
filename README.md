# Fibonacci Extensions for Python

This project is of no practical value, it's just for me to learn about building
python extensions and compare performance between languages. For this purpose,
I'll be writing a function that computes the nth Fibonacci number in four
languages:

1. Rust
2. Julia
3. Python (native)
4. Cython

Then I'll compare the runtime for each. I'll be calculating them recursively
because I'm not terribly concerned about the space complexity for this as I
don't intend to use this for any serious purpose.
