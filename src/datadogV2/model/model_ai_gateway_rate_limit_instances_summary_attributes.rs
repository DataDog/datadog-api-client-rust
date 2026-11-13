// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregate of a template rate limit's counters.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRateLimitInstancesSummaryAttributes {
    /// Number of counters.
    #[serde(rename = "instance_count")]
    pub instance_count: i64,
    /// Requests the counters rejected in the last 15 minutes (ENFORCE), totaled over the counters whose usage was read.
    #[serde(rename = "limited_requests")]
    pub limited_requests: Option<i64>,
    /// Counters that limited, or would have limited, a request in the last 15 minutes.
    #[serde(rename = "limiting_count")]
    pub limiting_count: i64,
    /// Utilization of the fullest counter.
    #[serde(rename = "max_utilization")]
    pub max_utilization: f64,
    /// Counters at 80% or more of a limit in the rolling window.
    #[serde(rename = "near_limit_count")]
    pub near_limit_count: i64,
    /// Rate limit UUID.
    #[serde(rename = "rate_limit_id")]
    pub rate_limit_id: uuid::Uuid,
    /// True when the caller can't see the template's selector because it lists a provider or model the caller can't access. The counters are then hidden, and every count is zero.
    #[serde(rename = "scope_restricted")]
    pub scope_restricted: Option<bool>,
    /// True when some recently active counters' usage wasn't read, so the counts may be low.
    #[serde(rename = "truncated")]
    pub truncated: bool,
    /// Requests the counters would have rejected in the last 15 minutes (ALERT), totaled over the counters whose usage was read.
    #[serde(rename = "would_limit_requests")]
    pub would_limit_requests: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRateLimitInstancesSummaryAttributes {
    pub fn new(
        instance_count: i64,
        limiting_count: i64,
        max_utilization: f64,
        near_limit_count: i64,
        rate_limit_id: uuid::Uuid,
        truncated: bool,
    ) -> AiGatewayRateLimitInstancesSummaryAttributes {
        AiGatewayRateLimitInstancesSummaryAttributes {
            instance_count,
            limited_requests: None,
            limiting_count,
            max_utilization,
            near_limit_count,
            rate_limit_id,
            scope_restricted: None,
            truncated,
            would_limit_requests: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn limited_requests(mut self, value: i64) -> Self {
        self.limited_requests = Some(value);
        self
    }

    pub fn scope_restricted(mut self, value: bool) -> Self {
        self.scope_restricted = Some(value);
        self
    }

    pub fn would_limit_requests(mut self, value: i64) -> Self {
        self.would_limit_requests = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayRateLimitInstancesSummaryAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRateLimitInstancesSummaryAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayRateLimitInstancesSummaryAttributesVisitor {
            type Value = AiGatewayRateLimitInstancesSummaryAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut instance_count: Option<i64> = None;
                let mut limited_requests: Option<i64> = None;
                let mut limiting_count: Option<i64> = None;
                let mut max_utilization: Option<f64> = None;
                let mut near_limit_count: Option<i64> = None;
                let mut rate_limit_id: Option<uuid::Uuid> = None;
                let mut scope_restricted: Option<bool> = None;
                let mut truncated: Option<bool> = None;
                let mut would_limit_requests: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "instance_count" => {
                            instance_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limited_requests" => {
                            if v.is_null() {
                                continue;
                            }
                            limited_requests =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limiting_count" => {
                            limiting_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_utilization" => {
                            max_utilization =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "near_limit_count" => {
                            near_limit_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rate_limit_id" => {
                            rate_limit_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope_restricted" => {
                            if v.is_null() {
                                continue;
                            }
                            scope_restricted =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "truncated" => {
                            truncated = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "would_limit_requests" => {
                            if v.is_null() {
                                continue;
                            }
                            would_limit_requests =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let instance_count =
                    instance_count.ok_or_else(|| M::Error::missing_field("instance_count"))?;
                let limiting_count =
                    limiting_count.ok_or_else(|| M::Error::missing_field("limiting_count"))?;
                let max_utilization =
                    max_utilization.ok_or_else(|| M::Error::missing_field("max_utilization"))?;
                let near_limit_count =
                    near_limit_count.ok_or_else(|| M::Error::missing_field("near_limit_count"))?;
                let rate_limit_id =
                    rate_limit_id.ok_or_else(|| M::Error::missing_field("rate_limit_id"))?;
                let truncated = truncated.ok_or_else(|| M::Error::missing_field("truncated"))?;

                let content = AiGatewayRateLimitInstancesSummaryAttributes {
                    instance_count,
                    limited_requests,
                    limiting_count,
                    max_utilization,
                    near_limit_count,
                    rate_limit_id,
                    scope_restricted,
                    truncated,
                    would_limit_requests,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRateLimitInstancesSummaryAttributesVisitor)
    }
}
