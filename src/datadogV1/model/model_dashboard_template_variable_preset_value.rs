// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Template variables saved views.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardTemplateVariablePresetValue {
    /// The name of the variable.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// (deprecated) The value of the template variable within the saved view. Cannot be used in conjunction with `values`.
    #[deprecated]
    #[serde(rename = "value")]
    pub value: Option<String>,
    /// One or many template variable values within the saved view, which will be unioned together using `OR` if more than one is specified. Cannot be used in conjunction with `value`.
    #[serde(rename = "values")]
    pub values: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardTemplateVariablePresetValue {
    pub fn new() -> DashboardTemplateVariablePresetValue {
        #[allow(deprecated)]
        DashboardTemplateVariablePresetValue {
            name: None,
            value: None,
            values: None,
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn values(mut self, value: Vec<String>) -> Self {
        self.values = Some(value);
        self
    }
}

impl Default for DashboardTemplateVariablePresetValue {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardTemplateVariablePresetValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardTemplateVariablePresetValueVisitor;
        impl<'a> Visitor<'a> for DashboardTemplateVariablePresetValueVisitor {
            type Value = DashboardTemplateVariablePresetValue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut value: Option<String> = None;
                let mut values: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                #[allow(deprecated)]
                let content = DashboardTemplateVariablePresetValue {
                    name,
                    value,
                    values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardTemplateVariablePresetValueVisitor)
    }
}
