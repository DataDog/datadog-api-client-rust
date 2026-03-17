// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Event batching settings
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAmazonS3GenericBatchSettings {
    /// Maximum batch size in bytes.
    #[serde(rename = "batch_size")]
    pub batch_size: Option<i64>,
    /// Maximum number of seconds to wait before flushing the batch.
    #[serde(rename = "timeout_secs")]
    pub timeout_secs: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineAmazonS3GenericBatchSettings {
    pub fn new() -> ObservabilityPipelineAmazonS3GenericBatchSettings {
        ObservabilityPipelineAmazonS3GenericBatchSettings {
            batch_size: None,
            timeout_secs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn batch_size(mut self, value: i64) -> Self {
        self.batch_size = Some(value);
        self
    }

    pub fn timeout_secs(mut self, value: i64) -> Self {
        self.timeout_secs = Some(value);
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

impl Default for ObservabilityPipelineAmazonS3GenericBatchSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineAmazonS3GenericBatchSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAmazonS3GenericBatchSettingsVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAmazonS3GenericBatchSettingsVisitor {
            type Value = ObservabilityPipelineAmazonS3GenericBatchSettings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut batch_size: Option<i64> = None;
                let mut timeout_secs: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "batch_size" => {
                            if v.is_null() {
                                continue;
                            }
                            batch_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeout_secs" => {
                            if v.is_null() {
                                continue;
                            }
                            timeout_secs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ObservabilityPipelineAmazonS3GenericBatchSettings {
                    batch_size,
                    timeout_secs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineAmazonS3GenericBatchSettingsVisitor)
    }
}
