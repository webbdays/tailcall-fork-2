# Inline field

```graphql @config
schema {
  query: Query
}

type User @addField(name: "address", path: ["address", "geo", "lat"]) {
  address: Address @modify(omit: true)
}

type Address {
  geo: Geo
}

type Geo {
  lat: String
}

type Query {
  user: User @http(path: "/users/1", baseURL: "http://jsonplaceholder.typicode.com")
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
        geo:
          lat: "-37.3159"
      id: 1
      name: foo
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: query { user { address } }
```
