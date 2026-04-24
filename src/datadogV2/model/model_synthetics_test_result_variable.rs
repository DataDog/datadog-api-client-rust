// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A variable used or extracted during a test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultVariable {
    /// Error encountered when evaluating the variable.
    #[serde(rename = "err")]
    pub err: Option<String>,
    /// Human-readable error message for variable evaluation.
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// Example value for the variable.
    #[serde(rename = "example")]
    pub example: Option<String>,
    /// Variable identifier.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Variable name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Pattern used to extract the variable.
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    /// Whether the variable holds a secure value.
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    /// Variable type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Evaluated value of the variable.
    #[serde(rename = "val")]
    pub val: Option<String>,
    /// Current value of the variable.
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultVariable {
    pub fn new() -> SyntheticsTestResultVariable {
        SyntheticsTestResultVariable {
            err: None,
            error_message: None,
            example: None,
            id: None,
            name: None,
            pattern: None,
            secure: None,
            type_: None,
            val: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn err(mut self, value: String) -> Self {
        self.err = Some(value);
        self
    }

    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    pub fn example(mut self, value: String) -> Self {
        self.example = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn pattern(mut self, value: String) -> Self {
        self.pattern = Some(value);
        self
    }

    pub fn secure(mut self, value: bool) -> Self {
        self.secure = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn val(mut self, value: String) -> Self {
        self.val = Some(value);
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
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

impl Default for SyntheticsTestResultVariable {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultVariableVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultVariableVisitor {
            type Value = SyntheticsTestResultVariable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut err: Option<String> = None;
                let mut error_message: Option<String> = None;
                let mut example: Option<String> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut pattern: Option<String> = None;
                let mut secure: Option<bool> = None;
                let mut type_: Option<String> = None;
                let mut val: Option<String> = None;
                let mut value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "err" => {
                            if v.is_null() {
                                continue;
                            }
                            err = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_message" => {
                            if v.is_null() {
                                continue;
                            }
                            error_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "example" => {
                            if v.is_null() {
                                continue;
                            }
                            example = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pattern" => {
                            if v.is_null() {
                                continue;
                            }
                            pattern = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secure" => {
                            if v.is_null() {
                                continue;
                            }
                            secure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "val" => {
                            if v.is_null() {
                                continue;
                            }
                            val = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultVariable {
                    err,
                    error_message,
                    example,
                    id,
                    name,
                    pattern,
                    secure,
                    type_,
                    val,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultVariableVisitor)
    }
}
