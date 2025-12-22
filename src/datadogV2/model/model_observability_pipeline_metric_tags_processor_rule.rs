// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a rule for filtering metric tags based on key patterns.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineMetricTagsProcessorRule {
    /// The action to take on tags with matching keys.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::ObservabilityPipelineMetricTagsProcessorRuleAction,
    /// A list of tag keys to include or exclude.
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
    /// The processing mode for tag filtering.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::ObservabilityPipelineMetricTagsProcessorRuleMode,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineMetricTagsProcessorRule {
    pub fn new(
        action: crate::datadogV2::model::ObservabilityPipelineMetricTagsProcessorRuleAction,
        keys: Vec<String>,
        mode: crate::datadogV2::model::ObservabilityPipelineMetricTagsProcessorRuleMode,
    ) -> ObservabilityPipelineMetricTagsProcessorRule {
        ObservabilityPipelineMetricTagsProcessorRule {
            action,
            keys,
            mode,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineMetricTagsProcessorRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineMetricTagsProcessorRuleVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineMetricTagsProcessorRuleVisitor {
            type Value = ObservabilityPipelineMetricTagsProcessorRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<
                    crate::datadogV2::model::ObservabilityPipelineMetricTagsProcessorRuleAction,
                > = None;
                let mut keys: Option<Vec<String>> = None;
                let mut mode: Option<
                    crate::datadogV2::model::ObservabilityPipelineMetricTagsProcessorRuleMode,
                > = None;
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
                                    crate::datadogV2::model::ObservabilityPipelineMetricTagsProcessorRuleAction::UnparsedObject(_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "keys" => {
                            keys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::ObservabilityPipelineMetricTagsProcessorRuleMode::UnparsedObject(_mode) => {
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
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let keys = keys.ok_or_else(|| M::Error::missing_field("keys"))?;
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;

                let content = ObservabilityPipelineMetricTagsProcessorRule {
                    action,
                    keys,
                    mode,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineMetricTagsProcessorRuleVisitor)
    }
}
