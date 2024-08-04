---
identity: true
---

# test-http-baseurl

```graphql @config
schema @server @upstream(baseURL: "http://abc.com") {
  query: Query
}

type Query {
  bar: String @http(path: "/bar")
  foo: String @http(baseURL: "http://foo.com", path: "/foo")
}
```
