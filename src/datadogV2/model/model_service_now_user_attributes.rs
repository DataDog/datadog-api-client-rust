// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a ServiceNow user
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceNowUserAttributes {
    /// The email address of the user
    #[serde(rename = "email")]
    pub email: String,
    /// The full name of the user
    #[serde(rename = "full_name")]
    pub full_name: Option<String>,
    /// The ID of the ServiceNow instance
    #[serde(rename = "instance_id")]
    pub instance_id: uuid::Uuid,
    /// The username of the ServiceNow user
    #[serde(rename = "user_name")]
    pub user_name: String,
    /// The system ID of the user in ServiceNow
    #[serde(rename = "user_sys_id")]
    pub user_sys_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceNowUserAttributes {
    pub fn new(
        email: String,
        instance_id: uuid::Uuid,
        user_name: String,
        user_sys_id: String,
    ) -> ServiceNowUserAttributes {
        ServiceNowUserAttributes {
            email,
            full_name: None,
            instance_id,
            user_name,
            user_sys_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn full_name(mut self, value: String) -> Self {
        self.full_name = Some(value);
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

impl<'de> Deserialize<'de> for ServiceNowUserAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceNowUserAttributesVisitor;
        impl<'a> Visitor<'a> for ServiceNowUserAttributesVisitor {
            type Value = ServiceNowUserAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut email: Option<String> = None;
                let mut full_name: Option<String> = None;
                let mut instance_id: Option<uuid::Uuid> = None;
                let mut user_name: Option<String> = None;
                let mut user_sys_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "email" => {
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "full_name" => {
                            if v.is_null() {
                                continue;
                            }
                            full_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_id" => {
                            instance_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_name" => {
                            user_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_sys_id" => {
                            user_sys_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let email = email.ok_or_else(|| M::Error::missing_field("email"))?;
                let instance_id =
                    instance_id.ok_or_else(|| M::Error::missing_field("instance_id"))?;
                let user_name = user_name.ok_or_else(|| M::Error::missing_field("user_name"))?;
                let user_sys_id =
                    user_sys_id.ok_or_else(|| M::Error::missing_field("user_sys_id"))?;

                let content = ServiceNowUserAttributes {
                    email,
                    full_name,
                    instance_id,
                    user_name,
                    user_sys_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceNowUserAttributesVisitor)
    }
}
