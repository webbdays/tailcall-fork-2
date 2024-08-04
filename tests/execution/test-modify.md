---
identity: true
---

# test-modify

```graphql @config
schema @server @upstream(baseURL: "http://jsonplacheholder.typicode.com") {
  query: Query
}

input Foo {
  bar: String
}

type Query {
  foo(input: Foo): String @http(path: "/foo") @modify(name: "data")
}
```
