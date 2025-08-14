// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of the `HTTPBasicAuth` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HTTPBasicAuthUpdate {
    /// Password used for authentication. Saved in a secret store
    #[serde(rename = "password")]
    pub password: Option<String>,
    /// The definition of the `HTTPBasicAuth` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::HTTPBasicAuthType,
    /// Username used for authentication.
    #[serde(rename = "username")]
    pub username: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HTTPBasicAuthUpdate {
    pub fn new(type_: crate::datadogV2::model::HTTPBasicAuthType) -> HTTPBasicAuthUpdate {
        HTTPBasicAuthUpdate {
            password: None,
            type_,
            username: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn password(mut self, value: String) -> Self {
        self.password = Some(value);
        self
    }

    pub fn username(mut self, value: String) -> Self {
        self.username = Some(value);
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

impl<'de> Deserialize<'de> for HTTPBasicAuthUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HTTPBasicAuthUpdateVisitor;
        impl<'a> Visitor<'a> for HTTPBasicAuthUpdateVisitor {
            type Value = HTTPBasicAuthUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut password: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::HTTPBasicAuthType> = None;
                let mut username: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "password" => {
                            if v.is_null() {
                                continue;
                            }
                            password = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::HTTPBasicAuthType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "username" => {
                            if v.is_null() {
                                continue;
                            }
                            username = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = HTTPBasicAuthUpdate {
                    password,
                    type_,
                    username,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HTTPBasicAuthUpdateVisitor)
    }
}
