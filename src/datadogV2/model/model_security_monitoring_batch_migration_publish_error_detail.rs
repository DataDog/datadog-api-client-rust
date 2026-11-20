// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// A rule validation error.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringBatchMigrationPublishErrorDetail {
    /// The error code.
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// The error message.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The rule field the error applies to.
    #[serde(rename = "target")]
    pub target: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl SecurityMonitoringBatchMigrationPublishErrorDetail {
    pub fn new() -> SecurityMonitoringBatchMigrationPublishErrorDetail {
        SecurityMonitoringBatchMigrationPublishErrorDetail {
            code: None,
            message: None,
            target: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn code(mut self, value: String) -> Self {
        self.code = Some(value);
        self
    }
    
    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }
    
    pub fn target(mut self, value: String) -> Self {
        self.target = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for SecurityMonitoringBatchMigrationPublishErrorDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringBatchMigrationPublishErrorDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringBatchMigrationPublishErrorDetailVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringBatchMigrationPublishErrorDetailVisitor {
            type Value = SecurityMonitoringBatchMigrationPublishErrorDetail;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code: Option<String> = None;
                let mut message: Option<String> = None;
                let mut target: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code" => {
                            if v.is_null() {
                                continue;
                            }
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = SecurityMonitoringBatchMigrationPublishErrorDetail {
                    code,
                    message,
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringBatchMigrationPublishErrorDetailVisitor)
    }
}
