# Interactions compiler

POC in the continuation of the work done around [Scanning "modern" web applications with OWASP ZAP](https://blog.xaviermaso.com/2018/10/01/Scanning-modern-web-applications-with-OWASP-ZAP.html).

**Idea**:
After recording user interactions in a webpage, generate a script to drive a program to execute the exact same steps.


## Use

```sh
cat examples/record.json | cargo run -- -u https://website.com
```

## Contribute

```sh
# Install toolchain
rustup default stable
# Run the tests
cargo test
```
