// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Template variable metadata from a dashboard index.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReportScheduleIndexTemplateVariable {
    /// Available values for the template variable.
    #[serde(
        rename = "available_values",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub available_values: Option<Option<Vec<String>>>,
    /// Default values for the template variable.
    #[serde(
        rename = "defaults",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub defaults: Option<Option<Vec<String>>>,
    /// The template variable name.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// The tag prefix for the template variable, when available.
    #[serde(rename = "prefix", default, with = "::serde_with::rust::double_option")]
    pub prefix: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReportScheduleIndexTemplateVariable {
    pub fn new() -> ReportScheduleIndexTemplateVariable {
        ReportScheduleIndexTemplateVariable {
            available_values: None,
            defaults: None,
            name: None,
            prefix: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn available_values(mut self, value: Option<Vec<String>>) -> Self {
        self.available_values = Some(value);
        self
    }

    pub fn defaults(mut self, value: Option<Vec<String>>) -> Self {
        self.defaults = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = Some(value);
        self
    }

    pub fn prefix(mut self, value: Option<String>) -> Self {
        self.prefix = Some(value);
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

impl Default for ReportScheduleIndexTemplateVariable {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ReportScheduleIndexTemplateVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReportScheduleIndexTemplateVariableVisitor;
        impl<'a> Visitor<'a> for ReportScheduleIndexTemplateVariableVisitor {
            type Value = ReportScheduleIndexTemplateVariable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut available_values: Option<Option<Vec<String>>> = None;
                let mut defaults: Option<Option<Vec<String>>> = None;
                let mut name: Option<Option<String>> = None;
                let mut prefix: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "available_values" => {
                            available_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "defaults" => {
                            defaults = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefix" => {
                            prefix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ReportScheduleIndexTemplateVariable {
                    available_values,
                    defaults,
                    name,
                    prefix,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReportScheduleIndexTemplateVariableVisitor)
    }
}
