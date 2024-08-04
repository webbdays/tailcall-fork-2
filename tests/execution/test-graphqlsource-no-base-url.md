---
error: true
---

# test-graphqlsource-no-base-url

```graphql @config
schema {
  query: Query
}

type Post {
  id: Int!
  user: User @graphQL(name: "user", args: [{key: "id", value: "{{.value.userId}}"}])
}

type Query {
  post(id: Int!): Post @http(baseURL: "http://jsonplacheholder.typicode.com", path: "/posts/{{.args.id}}")
}

type User {
  id: Int
  name: String
}
```
