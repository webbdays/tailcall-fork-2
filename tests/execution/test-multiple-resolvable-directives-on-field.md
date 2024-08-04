---
error: true
---

# test-multiple-resolvable-directives-on-field

```graphql @config
schema @server @upstream(baseURL: "https://jsonplaceholder.typicode.com") {
  query: Query
}

type User {
  name: String
  id: Int
}

type Query {
  user1: User @expr(body: {name: "John"}) @http(path: "/users/1")
  user2: User @http(path: "/users/2") @call(steps: [{query: "something"}])
}
```
