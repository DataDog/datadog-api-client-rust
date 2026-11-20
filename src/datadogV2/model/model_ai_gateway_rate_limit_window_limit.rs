// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// A limit on one metric over a rolling window longer than a minute.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRateLimitWindowLimit {
    /// Maximum amount of the metric over the window. For `cost`, the amount is priced spend in microcents of 1e-8 USD each, as in `cost_microcents_per_minute`. For the same metric, a longer window must have a larger amount.
    #[serde(rename = "amount")]
    pub amount: i64,
    /// The metric that a window limit counts. `cost` counts priced spend in microcents.
    #[serde(rename = "metric")]
    pub metric: crate::datadogV2::model::AiGatewayRateLimitWindowLimitMetric,
    /// The rolling window that a limit applies to. For per-minute limits, use the `*_per_minute` fields instead.
    #[serde(rename = "window")]
    pub window: crate::datadogV2::model::AiGatewayRateLimitWindowLimitWindow,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AiGatewayRateLimitWindowLimit {
    pub fn new(amount: i64, metric: crate::datadogV2::model::AiGatewayRateLimitWindowLimitMetric, window: crate::datadogV2::model::AiGatewayRateLimitWindowLimitWindow) -> AiGatewayRateLimitWindowLimit {
        AiGatewayRateLimitWindowLimit {
            amount,
            metric,
            window,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for AiGatewayRateLimitWindowLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRateLimitWindowLimitVisitor;
        impl<'a> Visitor<'a> for AiGatewayRateLimitWindowLimitVisitor {
            type Value = AiGatewayRateLimitWindowLimit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut amount: Option<i64> = None;
                let mut metric: Option<crate::datadogV2::model::AiGatewayRateLimitWindowLimitMetric> = None;
                let mut window: Option<crate::datadogV2::model::AiGatewayRateLimitWindowLimitWindow> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "amount" => {
                            amount = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "metric" => {
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _metric) = metric {
                                match _metric {
                                    crate::datadogV2::model::AiGatewayRateLimitWindowLimitMetric::UnparsedObject(_metric) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "window" => {
                            window = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _window) = window {
                                match _window {
                                    crate::datadogV2::model::AiGatewayRateLimitWindowLimitWindow::UnparsedObject(_window) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let amount = amount.ok_or_else(|| M::Error::missing_field("amount"))?;
                let metric = metric.ok_or_else(|| M::Error::missing_field("metric"))?;
                let window = window.ok_or_else(|| M::Error::missing_field("window"))?;

                let content = AiGatewayRateLimitWindowLimit {
                    amount,
                    metric,
                    window,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRateLimitWindowLimitVisitor)
    }
}
