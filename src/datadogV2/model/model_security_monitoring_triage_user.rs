// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object representing a given user entity.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringTriageUser {
    /// The handle for this user account.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// Gravatar icon associated to the user.
    #[serde(rename = "icon")]
    pub icon: Option<String>,
    /// Numerical ID assigned by Datadog to this user account.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// The name for this user account.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// UUID assigned by Datadog to this user account.
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringTriageUser {
    pub fn new(uuid: String) -> SecurityMonitoringTriageUser {
        SecurityMonitoringTriageUser {
            handle: None,
            icon: None,
            id: None,
            name: None,
            uuid,
            _unparsed: false,
        }
    }

    pub fn handle(&mut self, value: String) -> &mut Self {
        self.handle = Some(value);
        self
    }

    pub fn icon(&mut self, value: String) -> &mut Self {
        self.icon = Some(value);
        self
    }

    pub fn id(&mut self, value: i64) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn name(&mut self, value: Option<String>) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringTriageUser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringTriageUserVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringTriageUserVisitor {
            type Value = SecurityMonitoringTriageUser;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut handle: Option<String> = None;
                let mut icon: Option<String> = None;
                let mut id: Option<i64> = None;
                let mut name: Option<Option<String>> = None;
                let mut uuid: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "handle" => {
                            if v.is_null() {
                                continue;
                            }
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "icon" => {
                            if v.is_null() {
                                continue;
                            }
                            icon = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uuid" => {
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let uuid = uuid.ok_or_else(|| M::Error::missing_field("uuid"))?;

                let content = SecurityMonitoringTriageUser {
                    handle,
                    icon,
                    id,
                    name,
                    uuid,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringTriageUserVisitor)
    }
}
