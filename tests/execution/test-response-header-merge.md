# test-response-header-value

```graphql @config
schema @server(headers: {custom: [{key: "a", value: "a"}]}) {
  query: Query
}

type User {
  name: String
  age: Int
}

type Query {
  user: User @expr(body: {name: "John"})
}
```

```graphql @config
schema @server(headers: {custom: [{key: "a", value: "b"}]}) {
  query: Query
}

type User {
  name: String
  age: Int
}

type Query {
  user: User @expr(body: {name: "John"})
}
```
