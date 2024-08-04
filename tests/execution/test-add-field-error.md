---
error: true
---

# test-add-field-error

```graphql @config
schema {
  query: Query
}

type User {
  name: String
  address: Address
}
type Address {
  city: String
}

type Query @addField(name: "street", path: ["user", "address", "street"]) {
  user: User @http(path: "/user/1", baseURL: "http://localhost:8000")
}
```
