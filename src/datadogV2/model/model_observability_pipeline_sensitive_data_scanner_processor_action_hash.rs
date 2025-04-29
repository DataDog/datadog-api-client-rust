// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for hashing matched sensitive values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSensitiveDataScannerProcessorActionHash {
    /// Action type that replaces the matched sensitive data with a hashed representation, preserving structure while securing content.
    #[serde(rename = "action")]
    pub action:
        crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionHashAction,
    /// The `ObservabilityPipelineSensitiveDataScannerProcessorActionHash` `options`.
    #[serde(rename = "options")]
    pub options: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineSensitiveDataScannerProcessorActionHash {
    pub fn new(
        action: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionHashAction,
    ) -> ObservabilityPipelineSensitiveDataScannerProcessorActionHash {
        ObservabilityPipelineSensitiveDataScannerProcessorActionHash {
            action,
            options: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn options(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.options = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSensitiveDataScannerProcessorActionHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSensitiveDataScannerProcessorActionHashVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSensitiveDataScannerProcessorActionHashVisitor {
            type Value = ObservabilityPipelineSensitiveDataScannerProcessorActionHash;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionHashAction> = None;
                let mut options: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _action) = action {
                                match _action {
                                    crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionHashAction::UnparsedObject(_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;

                let content = ObservabilityPipelineSensitiveDataScannerProcessorActionHash {
                    action,
                    options,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineSensitiveDataScannerProcessorActionHashVisitor)
    }
}
