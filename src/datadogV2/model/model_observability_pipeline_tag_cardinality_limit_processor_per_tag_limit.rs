// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A cardinality override for a specific tag key within a per-metric limit.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimit {
    /// How the per-tag override is applied. `limit_override` enforces a custom limit on the tag; `excluded` skips the tag from cardinality tracking.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorPerTagMode,
    /// The tag key this override applies to.
    #[serde(rename = "tag_key")]
    pub tag_key: String,
    /// The maximum number of distinct values allowed for this tag. Required when `mode` is `limit_override`. Must be omitted when `mode` is `excluded`.
    #[serde(rename = "value_limit")]
    pub value_limit: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimit {
    pub fn new(
        mode: crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorPerTagMode,
        tag_key: String,
    ) -> ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimit {
        ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimit {
            mode,
            tag_key,
            value_limit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn value_limit(mut self, value: i64) -> Self {
        self.value_limit = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimitVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimitVisitor {
            type Value = ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mode: Option<crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorPerTagMode> = None;
                let mut tag_key: Option<String> = None;
                let mut value_limit: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorPerTagMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tag_key" => {
                            tag_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value_limit" => {
                            if v.is_null() {
                                continue;
                            }
                            value_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let tag_key = tag_key.ok_or_else(|| M::Error::missing_field("tag_key"))?;

                let content = ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimit {
                    mode,
                    tag_key,
                    value_limit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimitVisitor)
    }
}
