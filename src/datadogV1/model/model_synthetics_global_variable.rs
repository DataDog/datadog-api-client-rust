// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Synthetic global variable.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsGlobalVariable {
    /// Attributes of the global variable.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV1::model::SyntheticsGlobalVariableAttributes>,
    /// Description of the global variable.
    #[serde(rename = "description")]
    pub description: String,
    /// Unique identifier of the global variable.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Determines if the global variable is a FIDO variable.
    #[serde(rename = "is_fido")]
    pub is_fido: Option<bool>,
    /// Determines if the global variable is a TOTP/MFA variable.
    #[serde(rename = "is_totp")]
    pub is_totp: Option<bool>,
    /// Name of the global variable. Unique across Synthetic global variables.
    #[serde(rename = "name")]
    pub name: String,
    /// Parser options to use for retrieving a Synthetic global variable from a Synthetic test. Used in conjunction with `parse_test_public_id`.
    #[serde(rename = "parse_test_options")]
    pub parse_test_options:
        Option<crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptions>,
    /// A Synthetic test ID to use as a test to generate the variable value.
    #[serde(rename = "parse_test_public_id")]
    pub parse_test_public_id: Option<String>,
    /// Tags of the global variable.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// Value of the global variable.
    #[serde(rename = "value")]
    pub value: crate::datadogV1::model::SyntheticsGlobalVariableValue,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsGlobalVariable {
    pub fn new(
        description: String,
        name: String,
        tags: Vec<String>,
        value: crate::datadogV1::model::SyntheticsGlobalVariableValue,
    ) -> SyntheticsGlobalVariable {
        SyntheticsGlobalVariable {
            attributes: None,
            description,
            id: None,
            is_fido: None,
            is_totp: None,
            name,
            parse_test_options: None,
            parse_test_public_id: None,
            tags,
            value,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV1::model::SyntheticsGlobalVariableAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn is_fido(mut self, value: bool) -> Self {
        self.is_fido = Some(value);
        self
    }

    pub fn is_totp(mut self, value: bool) -> Self {
        self.is_totp = Some(value);
        self
    }

    pub fn parse_test_options(
        mut self,
        value: crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptions,
    ) -> Self {
        self.parse_test_options = Some(value);
        self
    }

    pub fn parse_test_public_id(mut self, value: String) -> Self {
        self.parse_test_public_id = Some(value);
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

impl<'de> Deserialize<'de> for SyntheticsGlobalVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsGlobalVariableVisitor;
        impl<'a> Visitor<'a> for SyntheticsGlobalVariableVisitor {
            type Value = SyntheticsGlobalVariable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV1::model::SyntheticsGlobalVariableAttributes,
                > = None;
                let mut description: Option<String> = None;
                let mut id: Option<String> = None;
                let mut is_fido: Option<bool> = None;
                let mut is_totp: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut parse_test_options: Option<
                    crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptions,
                > = None;
                let mut parse_test_public_id: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut value: Option<crate::datadogV1::model::SyntheticsGlobalVariableValue> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_fido" => {
                            if v.is_null() {
                                continue;
                            }
                            is_fido = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_totp" => {
                            if v.is_null() {
                                continue;
                            }
                            is_totp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parse_test_options" => {
                            if v.is_null() {
                                continue;
                            }
                            parse_test_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parse_test_public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            parse_test_public_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = SyntheticsGlobalVariable {
                    attributes,
                    description,
                    id,
                    is_fido,
                    is_totp,
                    name,
                    parse_test_options,
                    parse_test_public_id,
                    tags,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsGlobalVariableVisitor)
    }
}
