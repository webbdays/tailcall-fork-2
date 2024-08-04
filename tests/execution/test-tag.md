---
identity: true
---

# test-tag

```graphql @config
schema @server @upstream(baseURL: "http://jsonplacheholder.typicode.com") {
  query: Query
}

type NEWS {
  getAllNews: News__NewsList!
}

type News__News @tag(id: "news.News") {
  body: String @expr(body: "This is a news body")
  id: Int @expr(body: 1)
  postImage: String @expr(body: "http://example.com/image.jpg")
  title: String @expr(body: "This is a news title")
}

type News__NewsList @tag(id: "news.NewsList") {
  news: [News__News]
}

type Query {
  news: NEWS
}
```
