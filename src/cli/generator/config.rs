use std::collections::BTreeMap;
use std::env;
use std::marker::PhantomData;
use std::path::Path;

use derive_setters::Setters;
use path_clean::PathClean;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::core::config::transformer::Preset;
use crate::core::config::{self, ConfigReaderContext};
use crate::core::mustache::Mustache;
use crate::core::valid::{Valid, ValidateFrom, Validator};

#[derive(Deserialize, Serialize, Debug, Default, Setters)]
#[serde(rename_all = "camelCase")]
pub struct Config<Status = UnResolved> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<Input<Status>>,
    pub output: Output<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<PresetConfig>,
    pub schema: Schema,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PresetConfig {
    merge_type: Option<f32>,
    #[serde(rename = "consolidateURL")]
    consolidate_url: Option<f32>,
    use_better_names: Option<bool>,
    tree_shake: Option<bool>,
    unwrap_single_field_types: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(transparent)]
pub struct Location<A>(
    #[serde(skip_serializing_if = "Location::is_empty")] pub String,
    #[serde(skip)] PhantomData<A>,
);

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct Headers<A>(
    #[serde(skip_serializing_if = "is_default")] pub Option<BTreeMap<String, String>>,
    #[serde(skip)] PhantomData<A>,
);

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Input<Status = UnResolved> {
    #[serde(flatten)]
    pub source: Source<Status>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Source<Status = UnResolved> {
    #[serde(rename_all = "camelCase")]
    Curl {
        src: Location<Status>,
        headers: Headers<Status>,
        field_name: String,
    },
    Proto {
        src: Location<Status>,
    },
    Config {
        src: Location<Status>,
    },
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Output<Status = UnResolved> {
    #[serde(skip_serializing_if = "Location::is_empty")]
    pub path: Location<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<config::Source>,
}

#[derive(Debug)]
pub enum Resolved {}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnResolved {}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

fn between(threshold: f32, min: f32, max: f32) -> Valid<(), String> {
    Valid::<(), String>::fail(format!(
        "Invalid threshold value ({:.2}). Allowed range is [{:.2} - {:.2}] inclusive.",
        threshold, min, max
    ))
    .when(|| !(min..=max).contains(&threshold))
}

impl ValidateFrom<PresetConfig> for Preset {
    type Error = String;
    fn validate_from(config: PresetConfig) -> Valid<Self, Self::Error> {
        let mut preset = Preset::new();

        if let Some(merge_type) = config.merge_type {
            preset = preset.merge_type(merge_type);
        }

        if let Some(consolidate_url) = config.consolidate_url {
            preset = preset.consolidate_url(consolidate_url);
        }

        if let Some(use_better_names) = config.use_better_names {
            preset = preset.use_better_names(use_better_names);
        }

        if let Some(unwrap_single_field_types) = config.unwrap_single_field_types {
            preset = preset.unwrap_single_field_types(unwrap_single_field_types);
        }

        if let Some(tree_shake) = config.tree_shake {
            preset = preset.tree_shake(tree_shake);
        }

        // TODO: The field names in trace should be inserted at compile time.
        Valid::succeed(preset)
            .and_then(|preset| {
                let merge_types_th = between(preset.merge_type, 0.0, 1.0).trace("mergeType");
                let consolidate_url_th =
                    between(preset.consolidate_url, 0.0, 1.0).trace("consolidateURL");

                merge_types_th.and(consolidate_url_th).map_to(preset)
            })
            .trace("preset")
    }
}

impl<A> Location<A> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Location<UnResolved> {
    fn into_resolved(self, parent_dir: Option<&Path>) -> Location<Resolved> {
        let path = {
            let path = self.0.as_str();
            if Url::parse(path).is_ok() || Path::new(path).is_absolute() {
                path.to_string()
            } else {
                let parent_dir = parent_dir.unwrap_or(Path::new(""));
                let joined_path = parent_dir.join(path);
                if let Ok(abs_path) = std::fs::canonicalize(&joined_path) {
                    abs_path.to_string_lossy().to_string()
                } else if let Ok(cwd) = env::current_dir() {
                    cwd.join(joined_path).clean().to_string_lossy().to_string()
                } else {
                    joined_path.clean().to_string_lossy().to_string()
                }
            }
        };
        Location(path, PhantomData)
    }
}

impl<A> Headers<A> {
    pub fn headers(&self) -> &Option<BTreeMap<String, String>> {
        &self.0
    }
}

impl Headers<UnResolved> {
    pub fn resolve(
        self,
        reader_context: &ConfigReaderContext,
    ) -> anyhow::Result<Headers<Resolved>> {
        // Resolve the header values with mustache template.
        let resolved_headers = if let Some(headers_inner) = self.0 {
            let mut resolved_headers = BTreeMap::new();
            for (key, value) in headers_inner.into_iter() {
                let template = Mustache::parse(&value)?;
                let resolved_value = template.render(reader_context);
                resolved_headers.insert(key, resolved_value);
            }
            Some(resolved_headers)
        } else {
            None
        };

        Ok(Headers(resolved_headers, PhantomData))
    }
}

impl Output<UnResolved> {
    pub fn resolve(self, parent_dir: Option<&Path>) -> anyhow::Result<Output<Resolved>> {
        Ok(Output {
            format: self.format,
            path: self.path.into_resolved(parent_dir),
        })
    }
}

impl Source<UnResolved> {
    pub fn resolve(
        self,
        parent_dir: Option<&Path>,
        reader_context: &ConfigReaderContext,
    ) -> anyhow::Result<Source<Resolved>> {
        match self {
            Source::Curl { src, field_name, headers } => {
                let resolved_path = src.into_resolved(parent_dir);
                let resolved_headers = headers.resolve(reader_context)?;
                Ok(Source::Curl { src: resolved_path, field_name, headers: resolved_headers })
            }
            Source::Proto { src } => {
                let resolved_path = src.into_resolved(parent_dir);
                Ok(Source::Proto { src: resolved_path })
            }
            Source::Config { src } => {
                let resolved_path = src.into_resolved(parent_dir);
                Ok(Source::Config { src: resolved_path })
            }
        }
    }
}

impl Input<UnResolved> {
    pub fn resolve(
        self,
        parent_dir: Option<&Path>,
        reader_context: &ConfigReaderContext,
    ) -> anyhow::Result<Input<Resolved>> {
        let resolved_source = self.source.resolve(parent_dir, reader_context)?;
        Ok(Input { source: resolved_source })
    }
}

impl Config {
    /// Resolves all the relative paths present inside the GeneratorConfig.
    pub fn into_resolved(
        self,
        config_path: &str,
        reader_context: ConfigReaderContext,
    ) -> anyhow::Result<Config<Resolved>> {
        let parent_dir = Some(Path::new(config_path).parent().unwrap_or(Path::new("")));

        let inputs = self
            .inputs
            .into_iter()
            .map(|input| input.resolve(parent_dir, &reader_context))
            .collect::<anyhow::Result<Vec<Input<Resolved>>>>()?;

        let output = self.output.resolve(parent_dir)?;

        Ok(Config { inputs, output, schema: self.schema, preset: self.preset })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::core::tests::TestEnvIO;
    use crate::core::valid::{ValidateInto, ValidationError, Validator};

    fn location<S: AsRef<str>>(s: S) -> Location<UnResolved> {
        Location(s.as_ref().to_string(), PhantomData)
    }

    fn to_headers(raw_headers: BTreeMap<String, String>) -> Headers<UnResolved> {
        Headers(Some(raw_headers), PhantomData)
    }

    #[test]
    fn test_headers_resolve() {
        let mut headers = BTreeMap::new();
        headers.insert(
            "Authorization".to_owned(),
            "Bearer {{.env.TOKEN}}".to_owned(),
        );

        let mut env_vars = HashMap::new();
        let token = "eyJhbGciOiJIUzI1NiIsInR5";
        env_vars.insert("TOKEN".to_owned(), token.to_owned());

        let unresolved_headers = to_headers(headers);

        let mut runtime = crate::core::runtime::test::init(None);
        runtime.env = Arc::new(TestEnvIO::init(env_vars));

        let reader_ctx = ConfigReaderContext {
            runtime: &runtime,
            vars: &Default::default(),
            headers: Default::default(),
        };

        let resolved_headers = unresolved_headers.resolve(&reader_ctx).unwrap();

        let expected = format!("Bearer {token}");
        let result = resolved_headers
            .headers()
            .to_owned()
            .unwrap()
            .get("Authorization")
            .unwrap()
            .to_owned();
        assert_eq!(
            result, expected,
            "Authorization header should be resolved correctly"
        );
    }

    #[test]
    fn test_config_codec() {
        let mut headers = BTreeMap::new();
        headers.insert("user-agent".to_owned(), "tailcall-v1".to_owned());
        let config = Config::default().inputs(vec![Input {
            source: Source::Curl {
                src: location("https://example.com"),
                headers: to_headers(headers),
                field_name: "test".to_string(),
            },
        }]);
        let actual = serde_json::to_string_pretty(&config).unwrap();
        insta::assert_snapshot!(actual)
    }

    #[test]
    fn should_fail_when_invalid_merge_type_threshold() {
        let config_preset = PresetConfig {
            tree_shake: None,
            use_better_names: None,
            merge_type: Some(2.0),
            consolidate_url: None,
            unwrap_single_field_types: None,
        };

        let transform_preset: Result<Preset, ValidationError<String>> =
            config_preset.validate_into().to_result();
        assert!(transform_preset.is_err());
    }

    #[test]
    fn should_use_user_provided_presets_when_provided() {
        let config_preset = PresetConfig {
            tree_shake: Some(true),
            use_better_names: Some(true),
            merge_type: Some(0.5),
            consolidate_url: Some(1.0),
            unwrap_single_field_types: None,
        };
        let transform_preset: Preset = config_preset.validate_into().to_result().unwrap();
        let expected_preset = Preset::new()
            .use_better_names(true)
            .tree_shake(true)
            .consolidate_url(1.0)
            .merge_type(0.5);
        assert_eq!(transform_preset, expected_preset);
    }

    #[test]
    fn test_location_resolve_with_url() {
        let json_source = r#""https://dummyjson.com/products""#;
        let de_source: Location<UnResolved> = serde_json::from_str(json_source).unwrap();
        let de_source = de_source.into_resolved(None);
        assert_eq!(de_source.0, "https://dummyjson.com/products");
        assert_eq!(de_source.1, PhantomData::<Resolved>);
    }

    #[test]
    fn test_is_empty() {
        let location_empty: Location<UnResolved> = serde_json::from_str(r#""""#).unwrap();
        let location_non_empty: Location<UnResolved> =
            serde_json::from_str(r#""https://dummyjson.com/products""#).unwrap();
        assert!(location_empty.is_empty());
        assert!(!location_non_empty.is_empty());
    }
}
