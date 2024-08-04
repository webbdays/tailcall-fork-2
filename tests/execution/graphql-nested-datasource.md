# GraphQL datasource inside another graphQL datasource

```graphql @config
schema
  @server(port: 8001, queryValidation: false, hostname: "0.0.0.0")
  @upstream(baseURL: "http://upstream/graphql", httpCache: 42) {
  query: Query
}

type Query {
  a: [A] @graphQL(name: "a")
}

type A {
  id: Int!
  bid: Int!
  cid: Int!
  b: B @graphQL(name: "b", args: [{key: "id", value: "{{.value.bid}}"}])
  c: C @graphQL(name: "c", args: [{key: "id", value: "{{.value.cid}}"}])
}

type B {
  id: Int!
  y: String!
}

type C {
  id: Int!
  x: String!
}
```

```yml @mock
- request:
    method: POST
    url: http://upstream/graphql
    textBody: '{ "query": "query { a { id bid cid } }" }'
  response:
    status: 200
    body:
      data:
        a:
          - bid: 1
            cid: 1
            id: 1
          - bid: 1
            cid: 2
            id: 2
          - bid: 2
            cid: 1
            id: 3
          - bid: 2
            cid: 2
            id: 4
- request:
    method: POST
    url: http://upstream/graphql
    textBody: '{ "query": "query { b(id: 1) { y } }" }'
  response:
    status: 200
    body:
      data:
        b:
          y: y1
  expectedHits: 2

- request:
    method: POST
    url: http://upstream/graphql
    textBody: '{ "query": "query { b(id: 2) { y } }" }'
  response:
    status: 200
    body:
      data:
        b:
          y: y2
  expectedHits: 2

- request:
    method: POST
    url: http://upstream/graphql
    textBody: '{ "query": "query { c(id: 1) { x } }" }'
  response:
    status: 200
    body:
      data:
        c:
          x: x1
  expectedHits: 2

- request:
    method: POST
    url: http://upstream/graphql
    textBody: '{ "query": "query { c(id: 2) { x } }" }'
  response:
    status: 200
    body:
      data:
        c:
          x: x2
  expectedHits: 2
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: query { a { id bid cid b { y } c { x } } }
```
