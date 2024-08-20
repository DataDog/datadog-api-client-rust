// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The instructions for what to compute for this query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventsCompute {
    /// The type of aggregation that can be performed on events-based queries.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV2::model::EventsAggregation,
    /// Interval for compute in milliseconds.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    /// The "measure" attribute on which to perform the computation.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventsCompute {
    pub fn new(aggregation: crate::datadogV2::model::EventsAggregation) -> EventsCompute {
        EventsCompute {
            aggregation,
            interval: None,
            metric: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
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

impl<'de> Deserialize<'de> for EventsCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventsComputeVisitor;
        impl<'a> Visitor<'a> for EventsComputeVisitor {
            type Value = EventsCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV2::model::EventsAggregation> = None;
                let mut interval: Option<i64> = None;
                let mut metric: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV2::model::EventsAggregation::UnparsedObject(
                                        _aggregation,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "interval" => {
                            if v.is_null() {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            if v.is_null() {
                                continue;
                            }
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let aggregation =
                    aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;

                let content = EventsCompute {
                    aggregation,
                    interval,
                    metric,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventsComputeVisitor)
    }
}
