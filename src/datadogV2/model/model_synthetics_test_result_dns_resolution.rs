// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// DNS resolution details recorded during the test execution.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultDnsResolution {
    /// DNS resolution attempts made during the test.
    #[serde(rename = "attempts")]
    pub attempts: Option<Vec<std::collections::BTreeMap<String, String>>>,
    /// Resolved IP address for the target host.
    #[serde(rename = "resolved_ip")]
    pub resolved_ip: Option<String>,
    /// Resolved port for the target service.
    #[serde(rename = "resolved_port")]
    pub resolved_port: Option<String>,
    /// DNS server used for the resolution.
    #[serde(rename = "server")]
    pub server: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultDnsResolution {
    pub fn new() -> SyntheticsTestResultDnsResolution {
        SyntheticsTestResultDnsResolution {
            attempts: None,
            resolved_ip: None,
            resolved_port: None,
            server: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attempts(mut self, value: Vec<std::collections::BTreeMap<String, String>>) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn resolved_ip(mut self, value: String) -> Self {
        self.resolved_ip = Some(value);
        self
    }

    pub fn resolved_port(mut self, value: String) -> Self {
        self.resolved_port = Some(value);
        self
    }

    pub fn server(mut self, value: String) -> Self {
        self.server = Some(value);
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

impl Default for SyntheticsTestResultDnsResolution {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultDnsResolution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultDnsResolutionVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultDnsResolutionVisitor {
            type Value = SyntheticsTestResultDnsResolution;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attempts: Option<Vec<std::collections::BTreeMap<String, String>>> = None;
                let mut resolved_ip: Option<String> = None;
                let mut resolved_port: Option<String> = None;
                let mut server: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attempts" => {
                            if v.is_null() {
                                continue;
                            }
                            attempts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resolved_ip" => {
                            if v.is_null() {
                                continue;
                            }
                            resolved_ip =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resolved_port" => {
                            if v.is_null() {
                                continue;
                            }
                            resolved_port =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "server" => {
                            if v.is_null() {
                                continue;
                            }
                            server = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultDnsResolution {
                    attempts,
                    resolved_ip,
                    resolved_port,
                    server,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultDnsResolutionVisitor)
    }
}
