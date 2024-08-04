---
identity: true
---

# test-description-many

```graphql @config
schema @server @upstream(baseURL: "http://jsonplacheholder.typicode.com") {
  query: Query
}

type Bar {
  """
  This is test2
  """
  baz: String
}

type Query {
  """
  This is test
  """
  foo: Bar @http(path: "/foo")
}
```
