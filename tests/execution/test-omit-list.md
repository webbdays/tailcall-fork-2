---
identity: true
---

# test-omit-list

```graphql @config
schema @server @upstream(baseURL: "http://jsonplacheholder.typicode.com") {
  query: Query
}

type A {
  b: B
}

type B {
  c: String
}

type Foo {
  a: A
}

type Query @addField(name: "foo", path: ["foo", "a", "0", "b"]) {
  foo: [Foo] @http(path: "/foo") @omit
}
```
