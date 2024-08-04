use std::str::FromStr;

use hyper::header::{HeaderName, HeaderValue};
use hyper::HeaderMap;
use url::Url;

use super::TryFoldConfig;
use crate::core::config::{
    self, Apollo, ConfigModule, KeyValue, PrometheusExporter, StdoutExporter,
};
use crate::core::directive::DirectiveCodec;
use crate::core::try_fold::TryFold;
use crate::core::valid::{Valid, ValidationError, Validator};

#[derive(Debug, Clone)]
pub struct OtlpExporter {
    pub url: Url,
    pub headers: HeaderMap,
}

#[derive(Debug, Clone)]
pub enum TelemetryExporter {
    Stdout(StdoutExporter),
    Otlp(OtlpExporter),
    Prometheus(PrometheusExporter),
    Apollo(Apollo),
}

#[derive(Debug, Default, Clone)]
pub struct Telemetry {
    pub export: Option<TelemetryExporter>,
    pub request_headers: Vec<String>,
}

fn to_url(url: &str) -> Valid<Url, String> {
    Valid::from(Url::parse(url).map_err(|e| ValidationError::new(e.to_string()))).trace("url")
}

fn to_headers(headers: Vec<KeyValue>) -> Valid<HeaderMap, String> {
    Valid::from_iter(headers.iter(), |key_value| {
        Valid::from(
            HeaderName::from_str(&key_value.key)
                .map_err(|err| ValidationError::new(err.to_string())),
        )
        .zip(Valid::from(
            HeaderValue::from_str(&key_value.value)
                .map_err(|err| ValidationError::new(err.to_string())),
        ))
    })
    .map(HeaderMap::from_iter)
    .trace("headers")
}

pub fn to_opentelemetry<'a>() -> TryFold<'a, ConfigModule, Telemetry, String> {
    TryFoldConfig::<Telemetry>::new(|config, up| {
        if let Some(export) = config.telemetry.export.as_ref() {
            let export = match export {
                config::TelemetryExporter::Stdout(config) => {
                    Valid::succeed(TelemetryExporter::Stdout(config.clone()))
                }
                config::TelemetryExporter::Otlp(config) => to_url(&config.url)
                    .zip(to_headers(config.headers.clone()))
                    .map(|(url, headers)| TelemetryExporter::Otlp(OtlpExporter { url, headers }))
                    .trace("otlp"),
                config::TelemetryExporter::Prometheus(config) => {
                    Valid::succeed(TelemetryExporter::Prometheus(config.clone()))
                }
                config::TelemetryExporter::Apollo(apollo) => validate_apollo(apollo.clone())
                    .and_then(|apollo| Valid::succeed(TelemetryExporter::Apollo(apollo))),
            };

            export
                .map(|export| Telemetry {
                    export: Some(export),
                    request_headers: config.telemetry.request_headers.clone(),
                })
                .trace(config::Telemetry::trace_name().as_str())
        } else {
            Valid::succeed(up)
        }
    })
}

fn validate_apollo(apollo: Apollo) -> Valid<Apollo, String> {
    validate_graph_ref(&apollo.graph_ref)
        .map(|_| apollo)
        .trace("apollo.graph_ref")
}

fn validate_graph_ref(graph_ref: &str) -> Valid<(), String> {
    let is_valid = regex::Regex::new(r"^[A-Za-z0-9-_]+@[A-Za-z0-9-_]+$")
        .unwrap()
        .is_match(graph_ref);
    if is_valid {
        Valid::succeed(())
    } else {
        Valid::fail(format!("`graph_ref` should be in the format <graph_id>@<variant> where `graph_id` and `variant` can only contain letters, numbers, '-' and '_'. Found {graph_ref}").to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::validate_graph_ref;
    use crate::core::valid::Valid;

    #[test]
    fn test_validate_graph_ref() {
        let success = || Valid::succeed(());
        let failure = |graph_ref| {
            Valid::fail(format!("`graph_ref` should be in the format <graph_id>@<variant> where `graph_id` and `variant` can only contain letters, numbers, '-' and '_'. Found {graph_ref}").to_string())
        };

        assert_eq!(validate_graph_ref("graph_id@variant"), success());
        assert_eq!(
            validate_graph_ref("gr@ph_id@variant"),
            failure("gr@ph_id@variant")
        );
        assert_eq!(validate_graph_ref("graph-Id@variant"), success());
        assert_eq!(
            validate_graph_ref("graph$id@variant1"),
            failure("graph$id@variant1")
        );
        assert_eq!(
            validate_graph_ref("gr@ph_id@variant"),
            failure("gr@ph_id@variant")
        );
    }
}
