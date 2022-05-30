# strip_quantization
Simple project to try different type of encoding.

## Setup
You need cargo to build and run this program.
You can install it using rustup: https://rustup.rs/

To run this project locally, compile it using cargo:
```
cargo build --release
````

## Code examples
Apply quantization on 'file' and put output in 'output'. You can specify amount of colors with 'colors' argument.
```
cargo run --release -- --file 'file' --output 'output' --colors 'colors'
```
