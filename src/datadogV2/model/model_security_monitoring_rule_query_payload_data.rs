// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Payload used to test the rule query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleQueryPayloadData {
    /// Source of the payload.
    #[serde(rename = "ddsource")]
    pub ddsource: Option<String>,
    /// Tags associated with your data.
    #[serde(rename = "ddtags")]
    pub ddtags: Option<String>,
    /// The name of the originating host of the log.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// The message of the payload.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The name of the application or service generating the data.
    #[serde(rename = "service")]
    pub service: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleQueryPayloadData {
    pub fn new() -> SecurityMonitoringRuleQueryPayloadData {
        SecurityMonitoringRuleQueryPayloadData {
            ddsource: None,
            ddtags: None,
            hostname: None,
            message: None,
            service: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ddsource(mut self, value: String) -> Self {
        self.ddsource = Some(value);
        self
    }

    pub fn ddtags(mut self, value: String) -> Self {
        self.ddtags = Some(value);
        self
    }

    pub fn hostname(mut self, value: String) -> Self {
        self.hostname = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
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

impl Default for SecurityMonitoringRuleQueryPayloadData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleQueryPayloadData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleQueryPayloadDataVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleQueryPayloadDataVisitor {
            type Value = SecurityMonitoringRuleQueryPayloadData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ddsource: Option<String> = None;
                let mut ddtags: Option<String> = None;
                let mut hostname: Option<String> = None;
                let mut message: Option<String> = None;
                let mut service: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ddsource" => {
                            if v.is_null() {
                                continue;
                            }
                            ddsource = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ddtags" => {
                            if v.is_null() {
                                continue;
                            }
                            ddtags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringRuleQueryPayloadData {
                    ddsource,
                    ddtags,
                    hostname,
                    message,
                    service,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleQueryPayloadDataVisitor)
    }
}
