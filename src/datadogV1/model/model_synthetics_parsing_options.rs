// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Parsing options for variables to extract.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsParsingOptions {
    /// When type is `http_header` or `grpc_metadata`, name of the header or metadatum to extract.
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// Name of the variable to extract.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Details of the parser to use for the global variable.
    #[serde(rename = "parser")]
    pub parser: Option<crate::datadogV1::model::SyntheticsVariableParser>,
    /// Determines whether or not the extracted value will be obfuscated.
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    /// Property of the Synthetic Test Response to extract into a local variable.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SyntheticsLocalVariableParsingOptionsType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsParsingOptions {
    pub fn new() -> SyntheticsParsingOptions {
        SyntheticsParsingOptions {
            field: None,
            name: None,
            parser: None,
            secure: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn field(mut self, value: String) -> Self {
        self.field = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn parser(mut self, value: crate::datadogV1::model::SyntheticsVariableParser) -> Self {
        self.parser = Some(value);
        self
    }

    pub fn secure(mut self, value: bool) -> Self {
        self.secure = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV1::model::SyntheticsLocalVariableParsingOptionsType,
    ) -> Self {
        self.type_ = Some(value);
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

impl Default for SyntheticsParsingOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsParsingOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsParsingOptionsVisitor;
        impl<'a> Visitor<'a> for SyntheticsParsingOptionsVisitor {
            type Value = SyntheticsParsingOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut field: Option<String> = None;
                let mut name: Option<String> = None;
                let mut parser: Option<crate::datadogV1::model::SyntheticsVariableParser> = None;
                let mut secure: Option<bool> = None;
                let mut type_: Option<
                    crate::datadogV1::model::SyntheticsLocalVariableParsingOptionsType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "field" => {
                            if v.is_null() {
                                continue;
                            }
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parser" => {
                            if v.is_null() {
                                continue;
                            }
                            parser = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsLocalVariableParsingOptionsType::UnparsedObject(_type_) => {
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

                let content = SyntheticsParsingOptions {
                    field,
                    name,
                    parser,
                    secure,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsParsingOptionsVisitor)
    }
}
