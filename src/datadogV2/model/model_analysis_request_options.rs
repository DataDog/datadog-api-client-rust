// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Analyzer options sent by browser-based clients. Accepted but ignored: the API always runs
/// the analysis with tree-sitter enabled and rule output logged.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnalysisRequestOptions {
    /// Whether to capture the output logged by rules during execution. Ignored by this operation.
    #[serde(rename = "log_output")]
    pub log_output: Option<bool>,
    /// Whether to use tree-sitter to parse the source code. Ignored by this operation.
    #[serde(rename = "use_tree_sitter")]
    pub use_tree_sitter: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnalysisRequestOptions {
    pub fn new() -> AnalysisRequestOptions {
        AnalysisRequestOptions {
            log_output: None,
            use_tree_sitter: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn log_output(mut self, value: bool) -> Self {
        self.log_output = Some(value);
        self
    }

    pub fn use_tree_sitter(mut self, value: bool) -> Self {
        self.use_tree_sitter = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for AnalysisRequestOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AnalysisRequestOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnalysisRequestOptionsVisitor;
        impl<'a> Visitor<'a> for AnalysisRequestOptionsVisitor {
            type Value = AnalysisRequestOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut log_output: Option<bool> = None;
                let mut use_tree_sitter: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "log_output" => {
                            if v.is_null() {
                                continue;
                            }
                            log_output = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "use_tree_sitter" => {
                            if v.is_null() {
                                continue;
                            }
                            use_tree_sitter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AnalysisRequestOptions {
                    log_output,
                    use_tree_sitter,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnalysisRequestOptionsVisitor)
    }
}
