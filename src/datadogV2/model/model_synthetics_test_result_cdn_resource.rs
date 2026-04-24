// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A CDN resource encountered while executing a browser step.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultCdnResource {
    /// CDN provider details inferred from response headers.
    #[serde(rename = "cdn")]
    pub cdn: Option<crate::datadogV2::model::SyntheticsTestResultCdnProviderInfo>,
    /// Resolved IP address for the CDN resource.
    #[serde(rename = "resolved_ip")]
    pub resolved_ip: Option<String>,
    /// Unix timestamp (ms) of when the resource was fetched.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
    /// Timing breakdown for fetching the CDN resource.
    #[serde(rename = "timings")]
    pub timings: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultCdnResource {
    pub fn new() -> SyntheticsTestResultCdnResource {
        SyntheticsTestResultCdnResource {
            cdn: None,
            resolved_ip: None,
            timestamp: None,
            timings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cdn(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultCdnProviderInfo,
    ) -> Self {
        self.cdn = Some(value);
        self
    }

    pub fn resolved_ip(mut self, value: String) -> Self {
        self.resolved_ip = Some(value);
        self
    }

    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn timings(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.timings = Some(value);
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

impl Default for SyntheticsTestResultCdnResource {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultCdnResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultCdnResourceVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultCdnResourceVisitor {
            type Value = SyntheticsTestResultCdnResource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cdn: Option<crate::datadogV2::model::SyntheticsTestResultCdnProviderInfo> =
                    None;
                let mut resolved_ip: Option<String> = None;
                let mut timestamp: Option<i64> = None;
                let mut timings: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cdn" => {
                            if v.is_null() {
                                continue;
                            }
                            cdn = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resolved_ip" => {
                            if v.is_null() {
                                continue;
                            }
                            resolved_ip =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timings" => {
                            if v.is_null() {
                                continue;
                            }
                            timings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultCdnResource {
                    cdn,
                    resolved_ip,
                    timestamp,
                    timings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultCdnResourceVisitor)
    }
}
