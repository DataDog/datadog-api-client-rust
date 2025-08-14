// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of the `AWSAssumeRole` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSAssumeRoleUpdate {
    /// AWS account the connection is created for.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// Pass true if the `external_id` should be regenerated.
    #[serde(rename = "generate_new_external_id")]
    pub generate_new_external_id: Option<bool>,
    /// Role to assume.
    #[serde(rename = "role")]
    pub role: Option<String>,
    /// The definition of the `AWSAssumeRole` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AWSAssumeRoleType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSAssumeRoleUpdate {
    pub fn new(type_: crate::datadogV2::model::AWSAssumeRoleType) -> AWSAssumeRoleUpdate {
        AWSAssumeRoleUpdate {
            account_id: None,
            generate_new_external_id: None,
            role: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn generate_new_external_id(mut self, value: bool) -> Self {
        self.generate_new_external_id = Some(value);
        self
    }

    pub fn role(mut self, value: String) -> Self {
        self.role = Some(value);
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

impl<'de> Deserialize<'de> for AWSAssumeRoleUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSAssumeRoleUpdateVisitor;
        impl<'a> Visitor<'a> for AWSAssumeRoleUpdateVisitor {
            type Value = AWSAssumeRoleUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut generate_new_external_id: Option<bool> = None;
                let mut role: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::AWSAssumeRoleType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "generate_new_external_id" => {
                            if v.is_null() {
                                continue;
                            }
                            generate_new_external_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role" => {
                            if v.is_null() {
                                continue;
                            }
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::AWSAssumeRoleType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AWSAssumeRoleUpdate {
                    account_id,
                    generate_new_external_id,
                    role,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSAssumeRoleUpdateVisitor)
    }
}
