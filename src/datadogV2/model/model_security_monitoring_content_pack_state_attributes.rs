// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a content pack state.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringContentPackStateAttributes {
    /// Type-specific details for a content pack state. The set of fields present depends
    /// on the content pack's `type`. When Cloud SIEM is inactive for the requesting organization, `onboarding` is returned instead of the content pack's usual type, such as `logs` or `vulnerability`.`
    #[serde(rename = "details")]
    pub details: crate::datadogV2::model::SecurityMonitoringContentPackStateDetails,
    /// The current operational status of a content pack.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::SecurityMonitoringContentPackStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringContentPackStateAttributes {
    pub fn new(
        details: crate::datadogV2::model::SecurityMonitoringContentPackStateDetails,
        status: crate::datadogV2::model::SecurityMonitoringContentPackStatus,
    ) -> SecurityMonitoringContentPackStateAttributes {
        SecurityMonitoringContentPackStateAttributes {
            details,
            status,
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

impl<'de> Deserialize<'de> for SecurityMonitoringContentPackStateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringContentPackStateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringContentPackStateAttributesVisitor {
            type Value = SecurityMonitoringContentPackStateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut details: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackStateDetails,
                > = None;
                let mut status: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackStatus,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "details" => {
                            details = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _details) = details {
                                match _details {
                                    crate::datadogV2::model::SecurityMonitoringContentPackStateDetails::UnparsedObject(_details) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::SecurityMonitoringContentPackStatus::UnparsedObject(_status) => {
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
                let details = details.ok_or_else(|| M::Error::missing_field("details"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = SecurityMonitoringContentPackStateAttributes {
                    details,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringContentPackStateAttributesVisitor)
    }
}
