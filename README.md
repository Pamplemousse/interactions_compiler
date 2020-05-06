# Interactions compiler

POC in the continuation of the work done around [Scanning "modern" web applications with OWASP ZAP](https://blog.xaviermaso.com/2018/10/01/Scanning-modern-web-applications-with-OWASP-ZAP.html).

**Idea**:
After recording user interactions in a webpage, generate a script to drive a program to execute the exact same steps.


## Use

```json
cat <<__EOF__ > test.json
[ { "topic": "dom-events"
  , "element":
    { "nodeName": "DIV"
    , "className": "class1 class2"
    , "id": "an-id"
    }
  , "event": "click"
  , "timestamp": 1586991500209
  }
, { "topic": "dom-events"
  , "element":
    { "nodeName": "INPUT"
    , "className": "class3 class4"
    , "id": "email"
    }
  , "event": "keydown"
  , "timestamp": 1586991504673
  }
]
__EOF__

test.json | cargo run -- -u https://website.com
```


## Contribute

  * Run the tests: `cargo test`.
