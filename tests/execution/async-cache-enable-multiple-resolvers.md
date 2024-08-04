# Async Cache Enabled

```graphql @config
schema
  @server(port: 8000, queryValidation: false, dedupe: true)
  @upstream(baseURL: "http://jsonplaceholder.typicode.com") {
  query: Query
}

type Query {
  posts: [Post] @http(path: "/posts?id=1")
}

type Post {
  id: Int!
  title: String
  body: String
  userId: Int!
  user: User @http(path: "/users/{{.value.userId}}")
  taggedUsers: [User] @http(path: "/taggedUsers/{{.value.id}}")
}

type User {
  id: Int
  name: String
}
```

```yml @mock
- request:
    method: GET
    url: http://jsonplaceholder.typicode.com/posts?id=1
  response:
    status: 200
    body:
      - id: 1
        userId: 1
      - id: 1
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
- request:
    method: GET
    url: http://jsonplaceholder.typicode.com/taggedUsers/1
  expectedHits: 1
  response:
    status: 200
    body:
      - id: 2
        name: Ervin Howell
      - id: 3
        name: Clementine Bach
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: query { posts { user { name } taggedUsers } }
```
