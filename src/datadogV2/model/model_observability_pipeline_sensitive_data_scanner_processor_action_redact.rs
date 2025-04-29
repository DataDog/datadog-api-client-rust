// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for completely redacting matched sensitive data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSensitiveDataScannerProcessorActionRedact {
    /// Action type that completely replaces the matched sensitive data with a fixed replacement string to remove all visibility.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionRedactAction,
    /// Configuration for fully redacting sensitive data.
    #[serde(rename = "options")]
    pub options: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionRedactOptions,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineSensitiveDataScannerProcessorActionRedact {
    pub fn new(
        action: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionRedactAction,
        options: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionRedactOptions,
    ) -> ObservabilityPipelineSensitiveDataScannerProcessorActionRedact {
        ObservabilityPipelineSensitiveDataScannerProcessorActionRedact {
            action,
            options,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSensitiveDataScannerProcessorActionRedact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSensitiveDataScannerProcessorActionRedactVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSensitiveDataScannerProcessorActionRedactVisitor {
            type Value = ObservabilityPipelineSensitiveDataScannerProcessorActionRedact;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionRedactAction> = None;
                let mut options: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionRedactOptions> = None;
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
                                    crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionRedactAction::UnparsedObject(_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "options" => {
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
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;

                let content = ObservabilityPipelineSensitiveDataScannerProcessorActionRedact {
                    action,
                    options,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineSensitiveDataScannerProcessorActionRedactVisitor)
    }
}
