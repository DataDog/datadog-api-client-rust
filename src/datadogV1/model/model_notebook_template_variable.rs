// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Notebook template variable.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookTemplateVariable {
    /// The list of values that the template variable drop-down is limited to.
    #[serde(
        rename = "available_values",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub available_values: Option<Option<Vec<String>>>,
    /// Query used to dynamically populate the list of available values for the template variable.
    #[serde(rename = "available_values_query")]
    pub available_values_query:
        Option<crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQuery>,
    /// Mapping of data source names to template variable values.
    #[serde(rename = "data_source_mappings")]
    pub data_source_mappings: Option<std::collections::BTreeMap<String, String>>,
    /// (deprecated) The default value for the template variable on notebook load.
    /// Cannot be used in conjunction with `defaults`.
    #[deprecated]
    #[serde(
        rename = "default",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default: Option<Option<String>>,
    /// One or many default values for the template variable. Cannot be used in conjunction with `default`.
    #[serde(rename = "defaults")]
    pub defaults: Option<Vec<String>>,
    /// The name of the variable.
    #[serde(rename = "name")]
    pub name: String,
    /// The placement of the template variable in the notebook.
    #[serde(rename = "placement")]
    pub placement: Option<String>,
    /// The tag prefix associated with the variable. Only tags with this prefix appear in the variable drop-down.
    #[serde(rename = "prefix", default, with = "::serde_with::rust::double_option")]
    pub prefix: Option<Option<String>>,
    /// The type of the template variable.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookTemplateVariable {
    pub fn new(name: String) -> NotebookTemplateVariable {
        #[allow(deprecated)]
        NotebookTemplateVariable {
            available_values: None,
            available_values_query: None,
            data_source_mappings: None,
            default: None,
            defaults: None,
            name,
            placement: None,
            prefix: None,
            type_: None,
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn available_values(mut self, value: Option<Vec<String>>) -> Self {
        self.available_values = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn available_values_query(
        mut self,
        value: crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQuery,
    ) -> Self {
        self.available_values_query = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn data_source_mappings(
        mut self,
        value: std::collections::BTreeMap<String, String>,
    ) -> Self {
        self.data_source_mappings = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn default(mut self, value: Option<String>) -> Self {
        self.default = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn defaults(mut self, value: Vec<String>) -> Self {
        self.defaults = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn placement(mut self, value: String) -> Self {
        self.placement = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn prefix(mut self, value: Option<String>) -> Self {
        self.prefix = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for NotebookTemplateVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookTemplateVariableVisitor;
        impl<'a> Visitor<'a> for NotebookTemplateVariableVisitor {
            type Value = NotebookTemplateVariable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut available_values: Option<Option<Vec<String>>> = None;
                let mut available_values_query: Option<
                    crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQuery,
                > = None;
                let mut data_source_mappings: Option<std::collections::BTreeMap<String, String>> =
                    None;
                let mut default: Option<Option<String>> = None;
                let mut defaults: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut placement: Option<String> = None;
                let mut prefix: Option<Option<String>> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "available_values" => {
                            available_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "available_values_query" => {
                            if v.is_null() {
                                continue;
                            }
                            available_values_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _available_values_query) = available_values_query {
                                match _available_values_query {
                                    crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQuery::UnparsedObject(_available_values_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "data_source_mappings" => {
                            if v.is_null() {
                                continue;
                            }
                            data_source_mappings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default" => {
                            default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "defaults" => {
                            if v.is_null() {
                                continue;
                            }
                            defaults = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "placement" => {
                            if v.is_null() {
                                continue;
                            }
                            placement = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefix" => {
                            prefix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                #[allow(deprecated)]
                let content = NotebookTemplateVariable {
                    available_values,
                    available_values_query,
                    data_source_mappings,
                    default,
                    defaults,
                    name,
                    placement,
                    prefix,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookTemplateVariableVisitor)
    }
}
