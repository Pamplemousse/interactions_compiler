# Interactions compiler

POC in the continuation of the work done around [Scanning "modern" web applications with OWASP ZAP](https://blog.xaviermaso.com/2018/10/01/Scanning-modern-web-applications-with-OWASP-ZAP.html).

**Idea**:
After recording user interactions in a webpage, generate a script to drive a program to execute the exact same steps.

## Use

### As a CLI

```sh
cat examples/record.json | cargo run -- -u https://website.com
```

### In a webpage

```sh
wasm-pack build --target web
python3 -m http.serve --bind 127.0.0.1
# then visit http://localhost:8000/examples/index.html
```

## Contribute

```sh
# Install toolchain
rustup default stable
cargo install wasm-pack
# Run the tests
cargo test
```
