---
error: true
---

# test-grpc-invalid-proto-id

```graphql @config
schema {
  query: Query
}

type Query {
  news: NewsData @grpc(method: "abc.NewsService.GetAllNews", baseURL: "http://localhost:4000")
}

type NewsData {
  news: [News]
}

type News {
  id: Int!
  title: String!
  body: String!
  postImage: String!
}
```
