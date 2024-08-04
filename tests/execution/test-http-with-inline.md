---
error: true
---

# test-http-with-inline

```graphql @config
schema @server @upstream(baseURL: "http://jsonplaceholder.typicode.com") {
  query: Query
}

type Query @addField(name: "username", path: ["post", "user", "name"]) {
  post: Post @http(path: "/posts/1") @http(path: "/users/{{.value.userId}}")
}

type Post {
  id: Int
  title: String
  body: String
  userId: Int!
  user: User @http(path: "/users/{{.value.userId}}")
}

type User {
  id: Int
  name: String
}
```
