# n + 1 Request

```graphql @config
schema @upstream(baseURL: "http://example.com", batch: {delay: 1, maxSize: 1000}) {
  query: Query
}

type Query {
  foos: [Foo] @http(path: "/foos")
  bars: [Bar] @http(path: "/bars")
}

type Foo {
  id: Int!
  name: String!
  bar: Bar @http(path: "/bars", query: [{key: "fooId", value: "{{.value.id}}"}], batchKey: ["fooId"])
}

type Bar {
  id: Int!
  fooId: Int!
  foo: [Foo] @http(path: "/foos", query: [{key: "id", value: "{{.value.fooId}}"}], batchKey: ["id"])
}
```

```yml @mock
- request:
    method: GET
    url: http://example.com/foos
  response:
    status: 200
    body:
      - id: 1
        name: foo1
      - id: 2
        name: foo2
- request:
    method: GET
    url: http://example.com/bars?fooId=1&fooId=2
  response:
    status: 200
    body:
      - fooId: "1"
        id: 1
      - fooId: "2"
        id: 2
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: query { foos { bar {fooId id} id name} }
```
