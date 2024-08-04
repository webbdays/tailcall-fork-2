---
identity: true
---

# inline-many

```graphql @config
schema @server @upstream {
  query: Query
}

type Address {
  city: String
  street: String
  zipcode: String
}

type Query {
  user: User @http(baseURL: "http://jsonplaceholder.typicode.com", path: "/users/1")
}

type User
  @addField(name: "city", path: ["address", "city"])
  @addField(name: "street", path: ["address", "street"])
  @addField(name: "zipcode", path: ["address", "zipcode"]) {
  address: Address @modify(omit: true)
  name: String
}
```
