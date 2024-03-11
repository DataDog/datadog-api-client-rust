// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object defining a variable that can be used in your browser test.
/// See the [Recording Steps documentation](<https://docs.datadoghq.com/synthetics/browser_tests/actions/?tab=testanelementontheactivepage#variables>).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBrowserVariable {
    /// Example for the variable.
    #[serde(rename = "example")]
    pub example: Option<String>,
    /// ID for the variable. Global variables require an ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Name of the variable.
    #[serde(rename = "name")]
    pub name: String,
    /// Pattern of the variable.
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    /// Determines whether or not the browser test variable is obfuscated. Can only be used with browser variables of type `text`.
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    /// Type of browser test variable.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsBrowserVariableType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBrowserVariable {
    pub fn new(
        name: String,
        type_: crate::datadogV1::model::SyntheticsBrowserVariableType,
    ) -> SyntheticsBrowserVariable {
        SyntheticsBrowserVariable {
            example: None,
            id: None,
            name,
            pattern: None,
            secure: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn example(&mut self, value: String) -> &mut Self {
        self.example = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn pattern(&mut self, value: String) -> &mut Self {
        self.pattern = Some(value);
        self
    }

    pub fn secure(&mut self, value: bool) -> &mut Self {
        self.secure = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsBrowserVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBrowserVariableVisitor;
        impl<'a> Visitor<'a> for SyntheticsBrowserVariableVisitor {
            type Value = SyntheticsBrowserVariable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut example: Option<String> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut pattern: Option<String> = None;
                let mut secure: Option<bool> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsBrowserVariableType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsBrowserVariableType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsBrowserVariable {
                    example,
                    id,
                    name,
                    pattern,
                    secure,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBrowserVariableVisitor)
    }
}
