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
pub struct AWSAssumeRole {
    /// AWS account the connection is created for.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// External ID used to scope which connection can be used to assume the role.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Pass true if the `external_id` should be regenerated.
    #[serde(rename = "generate_new_external_id")]
    pub generate_new_external_id: Option<bool>,
    /// AWS account that will assume the role.
    #[serde(rename = "principal_id")]
    pub principal_id: Option<String>,
    /// Role to assume.
    #[serde(rename = "role")]
    pub role: String,
    /// The definition of the `AWSAssumeRole` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AWSAssumeRoleType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSAssumeRole {
    pub fn new(
        account_id: String,
        role: String,
        type_: crate::datadogV2::model::AWSAssumeRoleType,
    ) -> AWSAssumeRole {
        AWSAssumeRole {
            account_id,
            external_id: None,
            generate_new_external_id: None,
            principal_id: None,
            role,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn external_id(mut self, value: String) -> Self {
        self.external_id = Some(value);
        self
    }

    pub fn generate_new_external_id(mut self, value: bool) -> Self {
        self.generate_new_external_id = Some(value);
        self
    }

    pub fn principal_id(mut self, value: String) -> Self {
        self.principal_id = Some(value);
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

impl<'de> Deserialize<'de> for AWSAssumeRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSAssumeRoleVisitor;
        impl<'a> Visitor<'a> for AWSAssumeRoleVisitor {
            type Value = AWSAssumeRole;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut external_id: Option<String> = None;
                let mut generate_new_external_id: Option<bool> = None;
                let mut principal_id: Option<String> = None;
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
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external_id" => {
                            if v.is_null() {
                                continue;
                            }
                            external_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "generate_new_external_id" => {
                            if v.is_null() {
                                continue;
                            }
                            generate_new_external_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "principal_id" => {
                            if v.is_null() {
                                continue;
                            }
                            principal_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role" => {
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
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let role = role.ok_or_else(|| M::Error::missing_field("role"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AWSAssumeRole {
                    account_id,
                    external_id,
                    generate_new_external_id,
                    principal_id,
                    role,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSAssumeRoleVisitor)
    }
}
