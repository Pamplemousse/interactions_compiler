# Interactions compiler

POC in the continuation of the work done around [Scanning "modern" web applications with OWASP ZAP](https://blog.xaviermaso.com/2018/10/01/Scanning-modern-web-applications-with-OWASP-ZAP.html).

**Idea**:
After recording user interactions in a webpage, generate a script to drive a program to execute the exact same steps.


## Use

```
cat <<__EOF__ > test.json
{
  "michel": {
    "age": 42,
    "hobbies": ["peeling potatoes"]
  }
}
__EOF__
cat test.json | cargo run
```
