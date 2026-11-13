// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One counter of a template rate limit and its live usage.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRateLimitInstanceAttributes {
    /// When the gateway first recorded a request for this counter.
    #[serde(rename = "first_seen")]
    pub first_seen: chrono::DateTime<chrono::Utc>,
    /// Input tokens counted in the rolling one-minute window.
    #[serde(rename = "input_tokens")]
    pub input_tokens: i64,
    /// The rate limit's per-counter input token limit. Null when it doesn't limit input tokens.
    #[serde(
        rename = "input_tokens_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub input_tokens_per_minute: Option<Option<i64>>,
    /// Opaque canonical key identifying the counter.
    #[serde(rename = "instance_key")]
    pub instance_key: String,
    /// When the gateway last recorded a request for this counter; up to a few minutes behind live traffic.
    #[serde(rename = "last_seen")]
    pub last_seen: chrono::DateTime<chrono::Utc>,
    /// Requests this counter rejected in the last 15 minutes (ENFORCE).
    #[serde(rename = "limited_requests")]
    pub limited_requests: i64,
    /// Output tokens counted in the rolling one-minute window.
    #[serde(rename = "output_tokens")]
    pub output_tokens: i64,
    /// The rate limit's per-counter output token limit. Null when it doesn't limit output tokens.
    #[serde(
        rename = "output_tokens_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub output_tokens_per_minute: Option<Option<i64>>,
    /// Rate limit UUID.
    #[serde(rename = "rate_limit_id")]
    pub rate_limit_id: uuid::Uuid,
    /// Requests counted in the rolling one-minute window.
    #[serde(rename = "requests")]
    pub requests: i64,
    /// The rate limit's per-counter request limit. Null when it doesn't limit requests.
    #[serde(
        rename = "requests_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub requests_per_minute: Option<Option<i64>>,
    /// The counter's value for each per-value dimension. user_id and team_id values are raw UUIDs.
    #[serde(rename = "split_dims")]
    pub split_dims: std::collections::BTreeMap<String, String>,
    /// False when this counter's usage wasn't read because the request already read the most counters it may. Its usage then reads as zero.
    #[serde(rename = "usage_known")]
    pub usage_known: bool,
    /// The fullest of the rate limit's limits in the rolling window, as a fraction. 1 or more means the counter is at its limit.
    #[serde(rename = "utilization")]
    pub utilization: f64,
    /// Requests this counter would have rejected in the last 15 minutes (ALERT).
    #[serde(rename = "would_limit_requests")]
    pub would_limit_requests: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRateLimitInstanceAttributes {
    pub fn new(
        first_seen: chrono::DateTime<chrono::Utc>,
        input_tokens: i64,
        instance_key: String,
        last_seen: chrono::DateTime<chrono::Utc>,
        limited_requests: i64,
        output_tokens: i64,
        rate_limit_id: uuid::Uuid,
        requests: i64,
        split_dims: std::collections::BTreeMap<String, String>,
        usage_known: bool,
        utilization: f64,
        would_limit_requests: i64,
    ) -> AiGatewayRateLimitInstanceAttributes {
        AiGatewayRateLimitInstanceAttributes {
            first_seen,
            input_tokens,
            input_tokens_per_minute: None,
            instance_key,
            last_seen,
            limited_requests,
            output_tokens,
            output_tokens_per_minute: None,
            rate_limit_id,
            requests,
            requests_per_minute: None,
            split_dims,
            usage_known,
            utilization,
            would_limit_requests,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn input_tokens_per_minute(mut self, value: Option<i64>) -> Self {
        self.input_tokens_per_minute = Some(value);
        self
    }

    pub fn output_tokens_per_minute(mut self, value: Option<i64>) -> Self {
        self.output_tokens_per_minute = Some(value);
        self
    }

    pub fn requests_per_minute(mut self, value: Option<i64>) -> Self {
        self.requests_per_minute = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayRateLimitInstanceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRateLimitInstanceAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayRateLimitInstanceAttributesVisitor {
            type Value = AiGatewayRateLimitInstanceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut first_seen: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut input_tokens: Option<i64> = None;
                let mut input_tokens_per_minute: Option<Option<i64>> = None;
                let mut instance_key: Option<String> = None;
                let mut last_seen: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut limited_requests: Option<i64> = None;
                let mut output_tokens: Option<i64> = None;
                let mut output_tokens_per_minute: Option<Option<i64>> = None;
                let mut rate_limit_id: Option<uuid::Uuid> = None;
                let mut requests: Option<i64> = None;
                let mut requests_per_minute: Option<Option<i64>> = None;
                let mut split_dims: Option<std::collections::BTreeMap<String, String>> = None;
                let mut usage_known: Option<bool> = None;
                let mut utilization: Option<f64> = None;
                let mut would_limit_requests: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "first_seen" => {
                            first_seen = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "input_tokens" => {
                            input_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "input_tokens_per_minute" => {
                            input_tokens_per_minute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_key" => {
                            instance_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_seen" => {
                            last_seen = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limited_requests" => {
                            limited_requests =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output_tokens" => {
                            output_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output_tokens_per_minute" => {
                            output_tokens_per_minute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rate_limit_id" => {
                            rate_limit_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requests_per_minute" => {
                            requests_per_minute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "split_dims" => {
                            split_dims = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_known" => {
                            usage_known =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "utilization" => {
                            utilization =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "would_limit_requests" => {
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
                let first_seen = first_seen.ok_or_else(|| M::Error::missing_field("first_seen"))?;
                let input_tokens =
                    input_tokens.ok_or_else(|| M::Error::missing_field("input_tokens"))?;
                let instance_key =
                    instance_key.ok_or_else(|| M::Error::missing_field("instance_key"))?;
                let last_seen = last_seen.ok_or_else(|| M::Error::missing_field("last_seen"))?;
                let limited_requests =
                    limited_requests.ok_or_else(|| M::Error::missing_field("limited_requests"))?;
                let output_tokens =
                    output_tokens.ok_or_else(|| M::Error::missing_field("output_tokens"))?;
                let rate_limit_id =
                    rate_limit_id.ok_or_else(|| M::Error::missing_field("rate_limit_id"))?;
                let requests = requests.ok_or_else(|| M::Error::missing_field("requests"))?;
                let split_dims = split_dims.ok_or_else(|| M::Error::missing_field("split_dims"))?;
                let usage_known =
                    usage_known.ok_or_else(|| M::Error::missing_field("usage_known"))?;
                let utilization =
                    utilization.ok_or_else(|| M::Error::missing_field("utilization"))?;
                let would_limit_requests = would_limit_requests
                    .ok_or_else(|| M::Error::missing_field("would_limit_requests"))?;

                let content = AiGatewayRateLimitInstanceAttributes {
                    first_seen,
                    input_tokens,
                    input_tokens_per_minute,
                    instance_key,
                    last_seen,
                    limited_requests,
                    output_tokens,
                    output_tokens_per_minute,
                    rate_limit_id,
                    requests,
                    requests_per_minute,
                    split_dims,
                    usage_known,
                    utilization,
                    would_limit_requests,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRateLimitInstanceAttributesVisitor)
    }
}
