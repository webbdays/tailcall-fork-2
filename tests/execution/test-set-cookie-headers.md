# Set Cookie Header

```graphql @config
schema
  @server(port: 8080, hostname: "0.0.0.0", headers: {setCookies: true})
  @upstream(baseURL: "http://jsonplaceholder.typicode.com") {
  query: Query
}

type Query {
  user(id: Int!): User @http(path: "/users/{{.args.id}}")
}
type User {
  id: Int!
  name: String!
  username: String!
  email: String!
  phone: String
  website: String
}
```

```yaml @mock
- request:
    method: GET
    url: http://jsonplaceholder.typicode.com/users/1

  response:
    status: 200
    headers:
      set-cookie: user=1
    body:
      id: 1
      name: foo

- request:
    method: GET
    url: http://jsonplaceholder.typicode.com/users/2

  response:
    status: 200
    headers:
      set-cookie: user=2
    body:
      id: 2
      name: bar
```

```yaml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: "query { u1:user(id: 1) { name } u2:user(id: 2) { name } }"
```
