// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `UpdateAppRequestDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateAppRequestDataAttributes {
    /// The `attributes` `components`.
    #[serde(rename = "components")]
    pub components: Option<Vec<crate::datadogV2::model::ComponentGrid>>,
    /// The `attributes` `description`.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The `attributes` `embeddedQueries`.
    #[serde(rename = "embeddedQueries")]
    pub embedded_queries: Option<Vec<crate::datadogV2::model::Query>>,
    /// The definition of `InputSchema` object.
    #[serde(rename = "inputSchema")]
    pub input_schema: Option<crate::datadogV2::model::InputSchema>,
    /// The `attributes` `name`.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The `attributes` `rootInstanceName`.
    #[serde(rename = "rootInstanceName")]
    pub root_instance_name: Option<String>,
    /// The `attributes` `scripts`.
    #[serde(rename = "scripts")]
    pub scripts: Option<Vec<crate::datadogV2::model::Script>>,
    /// The `attributes` `tags`.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateAppRequestDataAttributes {
    pub fn new() -> UpdateAppRequestDataAttributes {
        UpdateAppRequestDataAttributes {
            components: None,
            description: None,
            embedded_queries: None,
            input_schema: None,
            name: None,
            root_instance_name: None,
            scripts: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn components(mut self, value: Vec<crate::datadogV2::model::ComponentGrid>) -> Self {
        self.components = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn embedded_queries(mut self, value: Vec<crate::datadogV2::model::Query>) -> Self {
        self.embedded_queries = Some(value);
        self
    }

    pub fn input_schema(mut self, value: crate::datadogV2::model::InputSchema) -> Self {
        self.input_schema = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn root_instance_name(mut self, value: String) -> Self {
        self.root_instance_name = Some(value);
        self
    }

    pub fn scripts(mut self, value: Vec<crate::datadogV2::model::Script>) -> Self {
        self.scripts = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl Default for UpdateAppRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UpdateAppRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateAppRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for UpdateAppRequestDataAttributesVisitor {
            type Value = UpdateAppRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut components: Option<Vec<crate::datadogV2::model::ComponentGrid>> = None;
                let mut description: Option<String> = None;
                let mut embedded_queries: Option<Vec<crate::datadogV2::model::Query>> = None;
                let mut input_schema: Option<crate::datadogV2::model::InputSchema> = None;
                let mut name: Option<String> = None;
                let mut root_instance_name: Option<String> = None;
                let mut scripts: Option<Vec<crate::datadogV2::model::Script>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "components" => {
                            if v.is_null() {
                                continue;
                            }
                            components = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "embeddedQueries" => {
                            if v.is_null() {
                                continue;
                            }
                            embedded_queries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputSchema" => {
                            if v.is_null() {
                                continue;
                            }
                            input_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rootInstanceName" => {
                            if v.is_null() {
                                continue;
                            }
                            root_instance_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scripts" => {
                            if v.is_null() {
                                continue;
                            }
                            scripts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UpdateAppRequestDataAttributes {
                    components,
                    description,
                    embedded_queries,
                    input_schema,
                    name,
                    root_instance_name,
                    scripts,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateAppRequestDataAttributesVisitor)
    }
}
