# Nested Recursive Type

```graphql @config
schema @server @upstream(baseURL: "https://jsonplaceholder.typicode.com") {
  query: Query
  mutation: Mutation
}

type User {
  name: String
  id: Int!
  connections: [Connection] @http(path: "/connections/{{.value.id}}")
}

type Connection {
  type: String
  nested: NestedUser
}

type NestedUser {
  user: User
}

type Query {
  user: User @http(path: "/users/1")
}

type Mutation {
  createUser(user: User): User @http(path: "/user", method: "POST", body: "{{.args.user}}")
}
```

```yml @mock
- request:
    method: GET
    url: https://jsonplaceholder.typicode.com/users/1
  response:
    status: 200
    body:
      id: 1
      name: User1
- request:
    method: GET
    url: https://jsonplaceholder.typicode.com/connections/1
  response:
    status: 200
    body:
      - type: friend
        nested:
          user:
            id: 2
            name: User2

- request:
    method: GET
    url: https://jsonplaceholder.typicode.com/connections/2
  response:
    status: 200
    body:
      - type: friend
        nested:
          user:
            id: 3
            name: User3
      - type: coworker
        nested:
          user:
            id: 4
            name: User4
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: |
      query {
        user {
          name
          id
          connections {
            type
            nested {
              user {
                name
                id
                connections {
                  nested {
                    user {
                      name
                      id
                    }
                  }
                }
              }
            }
          }
        }
      }
```
