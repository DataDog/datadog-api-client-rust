// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Contains information of the CI error.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppCIError {
    /// Error category used to differentiate between issues related to the developer or provider environments.
    #[serde(rename = "domain")]
    pub domain: Option<crate::datadogV2::model::CIAppCIErrorDomain>,
    /// Error message.
    #[serde(
        rename = "message",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub message: Option<Option<String>>,
    /// The stack trace of the reported errors.
    #[serde(rename = "stack", default, with = "::serde_with::rust::double_option")]
    pub stack: Option<Option<String>>,
    /// Short description of the error type.
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option")]
    pub type_: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppCIError {
    pub fn new() -> CIAppCIError {
        CIAppCIError {
            domain: None,
            message: None,
            stack: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn domain(mut self, value: crate::datadogV2::model::CIAppCIErrorDomain) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn message(mut self, value: Option<String>) -> Self {
        self.message = Some(value);
        self
    }

    pub fn stack(mut self, value: Option<String>) -> Self {
        self.stack = Some(value);
        self
    }

    pub fn type_(mut self, value: Option<String>) -> Self {
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

impl Default for CIAppCIError {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CIAppCIError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppCIErrorVisitor;
        impl<'a> Visitor<'a> for CIAppCIErrorVisitor {
            type Value = CIAppCIError;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut domain: Option<crate::datadogV2::model::CIAppCIErrorDomain> = None;
                let mut message: Option<Option<String>> = None;
                let mut stack: Option<Option<String>> = None;
                let mut type_: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "domain" => {
                            if v.is_null() {
                                continue;
                            }
                            domain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _domain) = domain {
                                match _domain {
                                    crate::datadogV2::model::CIAppCIErrorDomain::UnparsedObject(
                                        _domain,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stack" => {
                            stack = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CIAppCIError {
                    domain,
                    message,
                    stack,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppCIErrorVisitor)
    }
}
