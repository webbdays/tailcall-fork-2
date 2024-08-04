---
identity: true
---

# inline-many-list

```graphql @config
schema @server @upstream {
  query: Query
}

type A {
  b: [String]
  c: String
  d: String
}

type Query {
  u: U @http(baseURL: "http://jsonplaceholder.typicode.com", path: "/us/1")
}

type U
  @addField(name: "b", path: ["a", "b"])
  @addField(name: "c", path: ["a", "c"])
  @addField(name: "d", path: ["a", "d"]) {
  a: A @modify(omit: true)
  e: String
}
```
