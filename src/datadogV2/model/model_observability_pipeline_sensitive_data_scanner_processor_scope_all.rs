// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Applies scanning across all available fields.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSensitiveDataScannerProcessorScopeAll {
    /// Applies the rule to all fields.
    #[serde(rename = "target")]
    pub target:
        crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScopeAllTarget,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineSensitiveDataScannerProcessorScopeAll {
    pub fn new(
        target: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScopeAllTarget,
    ) -> ObservabilityPipelineSensitiveDataScannerProcessorScopeAll {
        ObservabilityPipelineSensitiveDataScannerProcessorScopeAll {
            target,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineSensitiveDataScannerProcessorScopeAll {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSensitiveDataScannerProcessorScopeAllVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSensitiveDataScannerProcessorScopeAllVisitor {
            type Value = ObservabilityPipelineSensitiveDataScannerProcessorScopeAll;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut target: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScopeAllTarget> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _target) = target {
                                match _target {
                                    crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScopeAllTarget::UnparsedObject(_target) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;

                let content = ObservabilityPipelineSensitiveDataScannerProcessorScopeAll {
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineSensitiveDataScannerProcessorScopeAllVisitor)
    }
}
