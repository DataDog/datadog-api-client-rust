// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configures how metrics are assigned to aggregation windows. When omitted, metrics are grouped using system time.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAggregateProcessorAggregationTiming {
    /// Grace period, in seconds, for late-arriving metrics when using event time. Defaults to 10 seconds when omitted.
    #[serde(rename = "allowed_lateness_secs")]
    pub allowed_lateness_secs: Option<i64>,
    /// Determines whether metrics are assigned to aggregation windows based on when they are processed or their timestamps.
    #[serde(rename = "type")]
    pub type_:
        crate::datadogV2::model::ObservabilityPipelineAggregateProcessorAggregationTimingType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineAggregateProcessorAggregationTiming {
    pub fn new(
        type_: crate::datadogV2::model::ObservabilityPipelineAggregateProcessorAggregationTimingType,
    ) -> ObservabilityPipelineAggregateProcessorAggregationTiming {
        ObservabilityPipelineAggregateProcessorAggregationTiming {
            allowed_lateness_secs: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allowed_lateness_secs(mut self, value: i64) -> Self {
        self.allowed_lateness_secs = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineAggregateProcessorAggregationTiming {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAggregateProcessorAggregationTimingVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAggregateProcessorAggregationTimingVisitor {
            type Value = ObservabilityPipelineAggregateProcessorAggregationTiming;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allowed_lateness_secs: Option<i64> = None;
                let mut type_: Option<crate::datadogV2::model::ObservabilityPipelineAggregateProcessorAggregationTimingType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allowed_lateness_secs" => {
                            if v.is_null() {
                                continue;
                            }
                            allowed_lateness_secs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineAggregateProcessorAggregationTimingType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineAggregateProcessorAggregationTiming {
                    allowed_lateness_secs,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineAggregateProcessorAggregationTimingVisitor)
    }
}
