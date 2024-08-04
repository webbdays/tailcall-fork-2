# Async Cache Disabled

```graphql @config
schema @server(port: 8000, queryValidation: false) @upstream(baseURL: "http://jsonplaceholder.typicode.com") {
  query: Query
}

type Query {
  posts: Post @http(path: "/post?id=1")
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

```yml @mock
- request:
    method: GET
    url: http://jsonplaceholder.typicode.com/post?id=1
  response:
    status: 200
    body:
      id: 1
      userId: 1
- request:
    method: GET
    url: http://jsonplaceholder.typicode.com/users/1
  expectedHits: 1
  response:
    status: 200
    body:
      id: 1
      name: Leanne Graham
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: query { posts { user { name } } }
```
