// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserInfo {
    /// The organization name.
    #[serde(rename = "orgName")]
    pub org_name: String,
    /// The user's email address.
    #[serde(rename = "userEmail")]
    pub user_email: String,
    /// The user's name.
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
    /// The user's UUID.
    #[serde(rename = "userUUID")]
    pub user_uuid: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserInfo {
    pub fn new(org_name: String, user_email: String, user_uuid: String) -> UserInfo {
        UserInfo {
            org_name,
            user_email,
            user_name: None,
            user_uuid,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn user_name(mut self, value: String) -> Self {
        self.user_name = Some(value);
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

impl<'de> Deserialize<'de> for UserInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserInfoVisitor;
        impl<'a> Visitor<'a> for UserInfoVisitor {
            type Value = UserInfo;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut org_name: Option<String> = None;
                let mut user_email: Option<String> = None;
                let mut user_name: Option<String> = None;
                let mut user_uuid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "orgName" => {
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "userEmail" => {
                            user_email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "userName" => {
                            if v.is_null() {
                                continue;
                            }
                            user_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "userUUID" => {
                            user_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let org_name = org_name.ok_or_else(|| M::Error::missing_field("org_name"))?;
                let user_email = user_email.ok_or_else(|| M::Error::missing_field("user_email"))?;
                let user_uuid = user_uuid.ok_or_else(|| M::Error::missing_field("user_uuid"))?;

                let content = UserInfo {
                    org_name,
                    user_email,
                    user_name,
                    user_uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserInfoVisitor)
    }
}
