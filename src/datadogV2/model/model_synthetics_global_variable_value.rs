// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Value of the global variable.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsGlobalVariableValue {
    /// Options for the Global Variable for MFA.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::SyntheticsGlobalVariableOptions>,
    /// Determines if the value of the variable is hidden.
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    /// Value of the global variable. When reading a global variable,
    /// the value will not be present if the variable is hidden with the `secure` property.
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsGlobalVariableValue {
    pub fn new() -> SyntheticsGlobalVariableValue {
        SyntheticsGlobalVariableValue {
            options: None,
            secure: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn options(
        mut self,
        value: crate::datadogV2::model::SyntheticsGlobalVariableOptions,
    ) -> Self {
        self.options = Some(value);
        self
    }

    pub fn secure(mut self, value: bool) -> Self {
        self.secure = Some(value);
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

impl Default for SyntheticsGlobalVariableValue {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsGlobalVariableValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsGlobalVariableValueVisitor;
        impl<'a> Visitor<'a> for SyntheticsGlobalVariableValueVisitor {
            type Value = SyntheticsGlobalVariableValue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut options: Option<crate::datadogV2::model::SyntheticsGlobalVariableOptions> =
                    None;
                let mut secure: Option<bool> = None;
                let mut value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secure" => {
                            if v.is_null() {
                                continue;
                            }
                            secure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = SyntheticsGlobalVariableValue {
                    options,
                    secure,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsGlobalVariableValueVisitor)
    }
}
