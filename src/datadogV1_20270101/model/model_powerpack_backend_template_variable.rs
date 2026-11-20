// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpackBackendTemplateVariable {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "prefix")]
    pub prefix: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "values")]
    pub values: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl PowerpackBackendTemplateVariable {
    pub fn new(name: String, values: Vec<String>) -> PowerpackBackendTemplateVariable {
        PowerpackBackendTemplateVariable {
            name,
            prefix: None,
            values,
            _unparsed: false,
        }
    }
    
    pub fn prefix(mut self, value: String) -> Self {
        self.prefix = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for PowerpackBackendTemplateVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackBackendTemplateVariableVisitor;
        impl<'a> Visitor<'a> for PowerpackBackendTemplateVariableVisitor {
            type Value = PowerpackBackendTemplateVariable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut prefix: Option<String> = None;
                let mut values: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "prefix" => {
                            if v.is_null() {
                                continue;
                            }
                            prefix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = PowerpackBackendTemplateVariable {
                    name,
                    prefix,
                    values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackBackendTemplateVariableVisitor)
    }
}
