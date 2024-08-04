# Resolved by parent

```graphql @config
schema {
  query: Query
}

type Address {
  street: String
}

type Query {
  user: User @http(path: "/users/1", baseURL: "http://jsonplaceholder.typicode.com")
}

type User @addField(name: "address", path: ["address", "street"]) {
  address: Address @modify(omit: true)
}
```

```yml @mock
- request:
    method: GET
    url: http://jsonplaceholder.typicode.com/users/1
  response:
    status: 200
    body:
      address:
        street: Kulas Light
      id: 1
      name: foo
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: query { user { address } }
```
