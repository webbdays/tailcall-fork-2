# Auth with JWT loaded from expr

```graphql @config
schema @server(port: 8000) @link(id: "jwks", type: Jwks, src: "jwks.json") {
  query: Query
  mutation: Mutation
}

type Query {
  scalar: String! @expr(body: "data from public scalar")
  protectedScalar: String! @protected @expr(body: "data from protected scalar")
  nested: Nested! @expr(body: {name: "nested name", protected: "protected nested"})
  protectedType: ProtectedType @expr(body: {name: "protected type name", nested: "protected type nested"})
}

type Mutation {
  protectedType: ProtectedType @http(baseURL: "http://upstream", path: "/protected")
}

type Nested {
  name: String!
  protected: String! @protected
}

type ProtectedType @protected {
  name: String!
  nested: String!
}
```

```json @file:jwks.json
{
  "keys": [
    {
      "kty": "RSA",
      "use": "sig",
      "alg": "RS256",
      "kid": "I48qMJp566SSKQogYXYtHBo9q6ZcEKHixNPeNoxV1c8",
      "n": "ksMb5oMlhJ_HzAebCuBG6-v5Qc4J111ur7Aux6-8SbxzqFONsf2Bw6ATG8pAfNeZ-USA3_T1mGkYTDvfoggXnxsduWV_lePZKKOq_Qp_EDdzic1bVTJQDad3CXldR3wV6UFDtMx6cCLXxPZM5n76e7ybPt0iNgwoGpJE28emMZJXrnEUFzxwFMq61UlzWEumYqW3uOUVp7r5XAF5jQ_1nQAnpHBnRFzdNPVb3E6odMGu3jgp8mkPbPMP16Fund4LVplLz8yrsE9TdVrSdYJThylRWn_BwvJ0DjUcp8ibJya86iClUlixAmBwR9NdStHwQqHwmMXMKkTXo-ytRmSUobzxX9T8ESkij6iBhQpmDMD3FbkK30Y7pUVEBBOyDfNcWOhholjOj9CRrxu9to5rc2wvufe24VlbKb9wngS_uGfK4AYvVyrcjdYMFkdqw-Mft14HwzdO2BTS0TeMDZuLmYhj_bu5_g2Zu6PH5OpIXF6Fi8_679pCG8wWAcFQrFrM0eA70wD_SqD_BXn6pWRpFXlcRy_7PWTZ3QmC7ycQFR6Wc6Px44y1xDUoq3rH0RlZkeicfvP6FRlpjFU7xF6LjAfd9ciYBZfJll6PE7zf-i_ZXEslv-tJ5-30-I4Slwj0tDrZ2Z54OgAg07AIwAiI5o4y-0vmuhUscNpfZsGAGhE",
      "e": "AQAB"
    }
  ]
}
```

```yml @mock
- request:
    method: GET
    url: http://upstream/protected
    headers:
      authorization: Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6Ikk0OHFNSnA1NjZTU0tRb2dZWFl0SEJvOXE2WmNFS0hpeE5QZU5veFYxYzgifQ.eyJleHAiOjIwMTkwNTY0NDEuMCwiaXNzIjoibWUiLCJzdWIiOiJ5b3UiLCJhdWQiOlsidGhlbSJdfQ.cU-hJgVGWxK3-IBggYBChhf3FzibBKjuDLtq2urJ99FVXIGZls0VMXjyNW7yHhLLuif_9t2N5UIUIq-hwXVv7rrGRPCGrlqKU0jsUH251Spy7_ppG5_B2LsG3cBJcwkD4AVz8qjT3AaE_vYZ4WnH-CQ-F5Vm7wiYZgbdyU8xgKoH85KAxaCdJJlYOi8mApE9_zcdmTNJrTNd9sp7PX3lXSUu9AWlrZkyO-HhVbXFunVtfduDuTeVXxP8iw1wt6171CFbPmQJU_b3xCornzyFKmhSc36yvlDfoPPclWmWeyOfFEp9lVhQm0WhfDK7GiuRtaOxD-tOvpTjpcoZBeJb7bSg2OsneyeM_33a0WoPmjHw8WIxbroJz_PrfE72_TzbcTSDttKAv_e75PE48Vvx0661miFv4Gq8RBzMl2G3pQMEVCOm83v7BpodfN_YVJcqZJjVHMA70TZQ4K3L4_i9sIK9jJFfwEDVM7nsDnUu96n4vKs1fVvAuieCIPAJrfNOUMy7TwLvhnhUARsKnzmtNNrJuDhhBx-X93AHcG3micXgnqkFdKn6-ZUZ63I2KEdmjwKmLTRrv4n4eZKrRN-OrHPI4gLxJUhmyPAHzZrikMVBcDYfALqyki5SeKkwd4v0JAm87QzR4YwMdKErr0Xa5JrZqHGe2TZgVO4hIc-KrPw
  response:
    status: 200
    body:
      name: mutation name
      nested: mutation nested
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: |
      query {
        scalar
        nested {
          name
        }
      }
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: |
      query {
        protectedScalar
      }
- method: POST
  url: http://localhost:8080/graphql
  headers:
    Authorization: Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6Ikk0OHFNSnA1NjZTU0tRb2dZWFl0SEJvOXE2WmNFS0hpeE5QZU5veFYxYzgifQ.eyJleHAiOjIwMTkwNTY0NDEuMCwiaXNzIjoibWUiLCJzdWIiOiJ5b3UiLCJhdWQiOlsidGhlbSJdfQ.cU-hJgVGWxK3-IBggYBChhf3FzibBKjuDLtq2urJ99FVXIGZls0VMXjyNW7yHhLLuif_9t2N5UIUIq-hwXVv7rrGRPCGrlqKU0jsUH251Spy7_ppG5_B2LsG3cBJcwkD4AVz8qjT3AaE_vYZ4WnH-CQ-F5Vm7wiYZgbdyU8xgKoH85KAxaCdJJlYOi8mApE9_zcdmTNJrTNd9sp7PX3lXSUu9AWlrZkyO-HhVbXFunVtfduDuTeVXxP8iw1wt6171CFbPmQJU_b3xCornzyFKmhSc36yvlDfoPPclWmWeyOfFEp9lVhQm0WhfDK7GiuRtaOxD-tOvpTjpcoZBeJb7bSg2OsneyeM_33a0WoPmjHw8WIxbroJz_PrfE72_TzbcTSDttKAv_e75PE48Vvx0661miFv4Gq8RBzMl2G3pQMEVCOm83v7BpodfN_YVJcqZJjVHMA70TZQ4K3L4_i9sIK9jJFfwEDVM7nsDnUu96n4vKs1fVvAuieCIPAJrfNOUMy7TwLvhnhUARsKnzmtNNrJuDhhBx-X93AHcG3micXgnqkFdKn6-ZUZ63I2KEdmjwKmLTRrv4n4eZKrRN-OrHPI4gLxJUhmyPAHzZrikMVBcDYfALqyki5SeKkwd4v0JAm87QzR4YwMdKErr0Xa5JrZqHGe2TZgVO4hIc-KrPw
  body:
    query: |
      query {
        protectedScalar
        nested {
          name
          protected
        }
        protectedType {
          name
          nested
        }
      }
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: |
      mutation {
        protectedType {
          name
          nested
        }
      }
- method: POST
  url: http://localhost:8080/graphql
  headers:
    Authorization: Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6Ikk0OHFNSnA1NjZTU0tRb2dZWFl0SEJvOXE2WmNFS0hpeE5QZU5veFYxYzgifQ.eyJleHAiOjIwMTkwNTY0NDEuMCwiaXNzIjoibWUiLCJzdWIiOiJ5b3UiLCJhdWQiOlsidGhlbSJdfQ.cU-hJgVGWxK3-IBggYBChhf3FzibBKjuDLtq2urJ99FVXIGZls0VMXjyNW7yHhLLuif_9t2N5UIUIq-hwXVv7rrGRPCGrlqKU0jsUH251Spy7_ppG5_B2LsG3cBJcwkD4AVz8qjT3AaE_vYZ4WnH-CQ-F5Vm7wiYZgbdyU8xgKoH85KAxaCdJJlYOi8mApE9_zcdmTNJrTNd9sp7PX3lXSUu9AWlrZkyO-HhVbXFunVtfduDuTeVXxP8iw1wt6171CFbPmQJU_b3xCornzyFKmhSc36yvlDfoPPclWmWeyOfFEp9lVhQm0WhfDK7GiuRtaOxD-tOvpTjpcoZBeJb7bSg2OsneyeM_33a0WoPmjHw8WIxbroJz_PrfE72_TzbcTSDttKAv_e75PE48Vvx0661miFv4Gq8RBzMl2G3pQMEVCOm83v7BpodfN_YVJcqZJjVHMA70TZQ4K3L4_i9sIK9jJFfwEDVM7nsDnUu96n4vKs1fVvAuieCIPAJrfNOUMy7TwLvhnhUARsKnzmtNNrJuDhhBx-X93AHcG3micXgnqkFdKn6-ZUZ63I2KEdmjwKmLTRrv4n4eZKrRN-OrHPI4gLxJUhmyPAHzZrikMVBcDYfALqyki5SeKkwd4v0JAm87QzR4YwMdKErr0Xa5JrZqHGe2TZgVO4hIc-KrPw
  body:
    query: |
      mutation {
        protectedType {
          name
          nested
        }
      }
- method: POST
  url: http://localhost:8080/graphql
  headers:
    Authorization: Bearer abc.def
  body:
    query: |
      query {
        protectedScalar
      }
```
