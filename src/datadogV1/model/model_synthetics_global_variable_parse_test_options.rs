// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Parser options to use for retrieving a Synthetic global variable from a Synthetic test. Used in conjunction with `parse_test_public_id`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsGlobalVariableParseTestOptions {
    /// When type is `http_header`, name of the header to use to extract the value.
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// When type is `local_variable`, name of the local variable to use to extract the value.
    #[serde(rename = "localVariableName")]
    pub local_variable_name: Option<String>,
    /// Details of the parser to use for the global variable.
    #[serde(rename = "parser")]
    pub parser: Option<crate::datadogV1::model::SyntheticsVariableParser>,
    /// Property of the Synthetic Test Response to use for a Synthetic global variable.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptionsType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsGlobalVariableParseTestOptions {
    pub fn new(
        type_: crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptionsType,
    ) -> SyntheticsGlobalVariableParseTestOptions {
        SyntheticsGlobalVariableParseTestOptions {
            field: None,
            local_variable_name: None,
            parser: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn field(&mut self, value: String) -> &mut Self {
        self.field = Some(value);
        self
    }

    pub fn local_variable_name(&mut self, value: String) -> &mut Self {
        self.local_variable_name = Some(value);
        self
    }

    pub fn parser(
        &mut self,
        value: crate::datadogV1::model::SyntheticsVariableParser,
    ) -> &mut Self {
        self.parser = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsGlobalVariableParseTestOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsGlobalVariableParseTestOptionsVisitor;
        impl<'a> Visitor<'a> for SyntheticsGlobalVariableParseTestOptionsVisitor {
            type Value = SyntheticsGlobalVariableParseTestOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut field: Option<String> = None;
                let mut local_variable_name: Option<String> = None;
                let mut parser: Option<crate::datadogV1::model::SyntheticsVariableParser> = None;
                let mut type_: Option<
                    crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptionsType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "field" => {
                            if v.is_null() {
                                continue;
                            }
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "localVariableName" => {
                            if v.is_null() {
                                continue;
                            }
                            local_variable_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parser" => {
                            if v.is_null() {
                                continue;
                            }
                            parser = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptionsType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsGlobalVariableParseTestOptions {
                    field,
                    local_variable_name,
                    parser,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsGlobalVariableParseTestOptionsVisitor)
    }
}
