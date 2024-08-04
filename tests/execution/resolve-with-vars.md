# Resolve with vars

```graphql @config
schema @server(vars: [{key: "id", value: "1"}]) {
  query: Query
}

type User {
  name: String
  id: Int
}

type Query {
  user: [User]
    @http(path: "/users", query: [{key: "id", value: "{{.vars.id}}"}], baseURL: "http://jsonplaceholder.typicode.com")
}
```

```yml @mock
- request:
    method: GET
    url: http://jsonplaceholder.typicode.com/users?id=1
  response:
    status: 200
    body:
      - id: 1
        name: Leanne Graham
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: "query { user(id: 1) { name } }"
```
