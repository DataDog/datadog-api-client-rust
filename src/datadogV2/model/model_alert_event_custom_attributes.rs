// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object representing custom alert event attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AlertEventCustomAttributes {
    /// Custom attributes. Support up to 100 properties and a maximum nesting depth of 10 levels.
    #[serde(rename = "custom")]
    pub custom: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The links related to the event. Maximum of 20 links allowed.
    #[serde(rename = "links")]
    pub links: Option<Vec<crate::datadogV2::model::AlertEventCustomAttributesLinksItems>>,
    /// The priority of the alert. Defaults to `5`.
    #[serde(rename = "priority")]
    pub priority: Option<crate::datadogV2::model::AlertEventCustomAttributesPriority>,
    /// The status of the alert.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::AlertEventCustomAttributesStatus,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AlertEventCustomAttributes {
    pub fn new(
        status: crate::datadogV2::model::AlertEventCustomAttributesStatus,
    ) -> AlertEventCustomAttributes {
        AlertEventCustomAttributes {
            custom: None,
            links: None,
            priority: None,
            status,
            _unparsed: false,
        }
    }

    pub fn custom(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.custom = Some(value);
        self
    }

    pub fn links(
        mut self,
        value: Vec<crate::datadogV2::model::AlertEventCustomAttributesLinksItems>,
    ) -> Self {
        self.links = Some(value);
        self
    }

    pub fn priority(
        mut self,
        value: crate::datadogV2::model::AlertEventCustomAttributesPriority,
    ) -> Self {
        self.priority = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AlertEventCustomAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AlertEventCustomAttributesVisitor;
        impl<'a> Visitor<'a> for AlertEventCustomAttributesVisitor {
            type Value = AlertEventCustomAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut links: Option<
                    Vec<crate::datadogV2::model::AlertEventCustomAttributesLinksItems>,
                > = None;
                let mut priority: Option<
                    crate::datadogV2::model::AlertEventCustomAttributesPriority,
                > = None;
                let mut status: Option<crate::datadogV2::model::AlertEventCustomAttributesStatus> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom" => {
                            if v.is_null() {
                                continue;
                            }
                            custom = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _priority) = priority {
                                match _priority {
                                    crate::datadogV2::model::AlertEventCustomAttributesPriority::UnparsedObject(_priority) => {
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
                                    crate::datadogV2::model::AlertEventCustomAttributesStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = AlertEventCustomAttributes {
                    custom,
                    links,
                    priority,
                    status,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AlertEventCustomAttributesVisitor)
    }
}
