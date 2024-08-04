---
error: true
---

# test-all-blueprint-errors

```graphql @config
schema @server {
  query: Query
  mutation: Mutation
}
type Mutation {
  a: String
}
type Query {
  foo(inp: B): Foo
  bar: String @expr @expr(body: {name: "John"})
}
type Foo {
  a: String @expr(body: "1")
  b: B
}
type B {
  a: String
}
```
