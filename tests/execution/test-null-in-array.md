# Empty Array Response

```graphql @config
schema @server {
  query: Query
}

type Query {
  hi(id: ID!): [Company] @http(baseURL: "http://localhost:3000", path: "/hi")
}
type Company {
  name: String
  id: ID
}
```

```yml @mock
- request:
    method: GET
    url: http://localhost:3000/hi
  response:
    status: 200
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: "query { hi (id: 1) { name id } }"
```
