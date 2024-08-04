use std::borrow::Cow;
use std::collections::BTreeMap;

use headers::HeaderMap;

use crate::core::has_headers::HasHeaders;
use crate::core::path::PathString;
use crate::core::runtime::TargetRuntime;

pub struct ConfigReaderContext<'a> {
    pub runtime: &'a TargetRuntime,
    pub vars: &'a BTreeMap<String, String>,
    pub headers: HeaderMap,
}

impl<'a> PathString for ConfigReaderContext<'a> {
    fn path_string<T: AsRef<str>>(&self, path: &[T]) -> Option<Cow<'_, str>> {
        if path.is_empty() {
            return None;
        }

        path.split_first()
            .and_then(|(head, tail)| match head.as_ref() {
                "vars" => self.vars.get(tail[0].as_ref()).map(|v| v.into()),
                "env" => self.runtime.env.get(tail[0].as_ref()),
                _ => None,
            })
    }
}

impl HasHeaders for ConfigReaderContext<'_> {
    fn headers(&self) -> &HeaderMap {
        &self.headers
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::core::tests::TestEnvIO;

    #[test]
    fn path_string() {
        let mut runtime = crate::core::runtime::test::init(None);
        runtime.env = Arc::new(TestEnvIO::from_iter([(
            "ENV_1".to_owned(),
            "ENV_VAL".to_owned(),
        )]));

        let reader_context = ConfigReaderContext {
            runtime: &runtime,
            vars: &BTreeMap::from_iter([("VAR_1".to_owned(), "VAR_VAL".to_owned())]),
            headers: Default::default(),
        };

        assert_eq!(
            reader_context.path_string(&["env", "ENV_1"]),
            Some("ENV_VAL".into())
        );
        assert_eq!(reader_context.path_string(&["env", "ENV_5"]), None);
        assert_eq!(
            reader_context.path_string(&["vars", "VAR_1"]),
            Some("VAR_VAL".into())
        );
        assert_eq!(reader_context.path_string(&["vars", "VAR_6"]), None);
        assert_eq!(reader_context.path_string(&["unknown", "unknown"]), None);
    }
}
