// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `throttle` processor limits the number of events that pass through over a given time window.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineThrottleProcessor {
    /// Whether this processor is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Optional list of fields used to group events before the threshold has been reached.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// The unique identifier for this processor.
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which logs this processor targets.
    #[serde(rename = "include")]
    pub include: String,
    /// the number of events allowed in a given time window. Events sent after the threshold has been reached, are dropped.
    #[serde(rename = "threshold")]
    pub threshold: i64,
    /// The processor type. The value should always be `throttle`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineThrottleProcessorType,
    /// The time window in seconds over which the threshold applies.
    #[serde(rename = "window")]
    pub window: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineThrottleProcessor {
    pub fn new(
        enabled: bool,
        id: String,
        include: String,
        threshold: i64,
        type_: crate::datadogV2::model::ObservabilityPipelineThrottleProcessorType,
        window: f64,
    ) -> ObservabilityPipelineThrottleProcessor {
        ObservabilityPipelineThrottleProcessor {
            enabled,
            group_by: None,
            id,
            include,
            threshold,
            type_,
            window,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn group_by(mut self, value: Vec<String>) -> Self {
        self.group_by = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineThrottleProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineThrottleProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineThrottleProcessorVisitor {
            type Value = ObservabilityPipelineThrottleProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut group_by: Option<Vec<String>> = None;
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut threshold: Option<i64> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineThrottleProcessorType,
                > = None;
                let mut window: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "threshold" => {
                            threshold = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineThrottleProcessorType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "window" => {
                            window = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let threshold = threshold.ok_or_else(|| M::Error::missing_field("threshold"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let window = window.ok_or_else(|| M::Error::missing_field("window"))?;

                let content = ObservabilityPipelineThrottleProcessor {
                    enabled,
                    group_by,
                    id,
                    include,
                    threshold,
                    type_,
                    window,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineThrottleProcessorVisitor)
    }
}
