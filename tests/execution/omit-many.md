---
identity: true
---

# omit-many

```graphql @config
schema @server @upstream {
  query: Query
}

type Address {
  city: String
  complements: [String]
  street: String
  zipcode: String
}

type Query {
  user: User @http(baseURL: "http://jsonplaceholder.typicode.com", path: "/users/1")
}

type User
  @addField(name: "zipcode", path: ["address", "zipcode"])
  @addField(name: "complements", path: ["address", "complements"]) {
  address: Address @omit
  name: String
}
```
