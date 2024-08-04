# Graphql datasource

```graphql @config
schema @upstream(batch: {delay: 1}) {
  query: Query
}

type Post {
  id: Int
  title: String
  userId: Int
  user: User
    @graphQL(
      args: [{key: "id", value: "{{.value.userId}}"}]
      baseURL: "http://upstream/graphql"
      batch: true
      name: "user"
    )
}

type User {
  id: Int
  name: String
}

type Query {
  posts: [Post] @http(path: "/posts", baseURL: "http://jsonplaceholder.typicode.com")
}
```

```yml @mock
- request:
    method: GET
    url: http://jsonplaceholder.typicode.com/posts
  response:
    status: 200
    body:
      - id: 1
        title: a
        userId: 1
      - id: 2
        title: b
        userId: 1
      - id: 3
        title: c
        userId: 2
      - id: 4
        title: d
        userId: 2
- request:
    method: POST
    url: http://upstream/graphql
    textBody: '[{ "query": "query { user(id: 1) { name } }" },{ "query": "query { user(id: 2) { name } }" }]'
  assertHits: false
  response:
    status: 200
    body:
      - data:
          user:
            name: Leanne Graham
      - data:
          user:
            name: Ervin Howell
- request:
    method: POST
    url: http://upstream/graphql
    textBody: '[{ "query": "query { user(id: 2) { name } }" },{ "query": "query { user(id: 1) { name } }" }]'
  assertHits: false
  response:
    status: 200
    body:
      - data:
          user:
            name: Ervin Howell
      - data:
          user:
            name: Leanne Graham
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: query { posts { title user { name } } }
```
