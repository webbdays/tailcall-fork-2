---
identity: true
---

# test-ref-other

```graphql @config
schema @server(port: 8000) @upstream(baseURL: "http://jsonplaceholder.typicode.com") {
  query: Query
}

type InPost {
  get: [Post] @http(path: "/posts")
}

type Post {
  id: Int!
  userId: Int!
}

type Query {
  posts: InPost
}
```
