// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Live consumption of one AI Gateway rate limit.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRateLimitUsageAttributes {
    /// Input tokens counted in the rolling one-minute window.
    #[serde(rename = "input_tokens")]
    pub input_tokens: i64,
    /// Requests this rate limit rejected in the last 15 minutes. Applies when mode is ENFORCE.
    #[serde(rename = "limited_requests")]
    pub limited_requests: i64,
    /// Output tokens counted in the rolling one-minute window.
    #[serde(rename = "output_tokens")]
    pub output_tokens: i64,
    /// Requests counted in the rolling one-minute window.
    #[serde(rename = "requests")]
    pub requests: i64,
    /// Requests this rate limit would have rejected in the last 15 minutes. Applies when mode is ALERT.
    #[serde(rename = "would_limit_requests")]
    pub would_limit_requests: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRateLimitUsageAttributes {
    pub fn new(
        input_tokens: i64,
        limited_requests: i64,
        output_tokens: i64,
        requests: i64,
        would_limit_requests: i64,
    ) -> AiGatewayRateLimitUsageAttributes {
        AiGatewayRateLimitUsageAttributes {
            input_tokens,
            limited_requests,
            output_tokens,
            requests,
            would_limit_requests,
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

impl<'de> Deserialize<'de> for AiGatewayRateLimitUsageAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRateLimitUsageAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayRateLimitUsageAttributesVisitor {
            type Value = AiGatewayRateLimitUsageAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut input_tokens: Option<i64> = None;
                let mut limited_requests: Option<i64> = None;
                let mut output_tokens: Option<i64> = None;
                let mut requests: Option<i64> = None;
                let mut would_limit_requests: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "input_tokens" => {
                            input_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limited_requests" => {
                            limited_requests =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output_tokens" => {
                            output_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let input_tokens =
                    input_tokens.ok_or_else(|| M::Error::missing_field("input_tokens"))?;
                let limited_requests =
                    limited_requests.ok_or_else(|| M::Error::missing_field("limited_requests"))?;
                let output_tokens =
                    output_tokens.ok_or_else(|| M::Error::missing_field("output_tokens"))?;
                let requests = requests.ok_or_else(|| M::Error::missing_field("requests"))?;
                let would_limit_requests = would_limit_requests
                    .ok_or_else(|| M::Error::missing_field("would_limit_requests"))?;

                let content = AiGatewayRateLimitUsageAttributes {
                    input_tokens,
                    limited_requests,
                    output_tokens,
                    requests,
                    would_limit_requests,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRateLimitUsageAttributesVisitor)
    }
}
