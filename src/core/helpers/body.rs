use serde_json::Value;

use crate::core::grpc::request_template::RequestBody;
use crate::core::mustache::Mustache;
use crate::core::valid::Valid;

pub fn to_body(body: Option<&Value>) -> Valid<Option<RequestBody>, String> {
    let Some(body) = body else {
        return Valid::succeed(None);
    };

    let mut req_body = RequestBody::default();

    let value = body.to_string();
    if let Ok(mustache) = Mustache::parse(&value) {
        req_body = req_body.mustache(Some(mustache));
    }
    Valid::succeed(Some(req_body.value(value)))
}

#[cfg(test)]
mod tests {
    use super::to_body;
    use crate::core::grpc::request_template::RequestBody;
    use crate::core::mustache::Mustache;
    use crate::core::valid::Valid;

    #[test]
    fn no_body() {
        let result = to_body(None);

        assert_eq!(result, Valid::succeed(None));
    }

    #[test]
    fn body_parse_success() {
        let value = serde_json::Value::String("content".to_string());
        let result = to_body(Some(&value));

        assert_eq!(
            result,
            Valid::succeed(Some(RequestBody {
                mustache: Some(Mustache::parse(value.to_string().as_str()).unwrap()),
                value: value.to_string()
            }))
        );
    }
}
