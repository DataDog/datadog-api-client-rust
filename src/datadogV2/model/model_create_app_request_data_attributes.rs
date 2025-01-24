// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// App definition attributes such as name, description, and components.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateAppRequestDataAttributes {
    /// The UI components that make up the app.
    #[serde(rename = "components")]
    pub components: Option<Vec<crate::datadogV2::model::ComponentGrid>>,
    /// A human-readable description for the app.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// An array of queries, such as external actions and state variables, that the app uses.
    #[serde(rename = "embeddedQueries")]
    pub embedded_queries: Option<Vec<crate::datadogV2::model::Query>>,
    /// The name of the app.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The name of the root component of the app. This must be a `grid` component that contains all other components.
    #[serde(rename = "rootInstanceName")]
    pub root_instance_name: Option<String>,
    /// A list of tags for the app, which can be used to filter apps.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateAppRequestDataAttributes {
    pub fn new() -> CreateAppRequestDataAttributes {
        CreateAppRequestDataAttributes {
            components: None,
            description: None,
            embedded_queries: None,
            name: None,
            root_instance_name: None,
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

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn root_instance_name(mut self, value: String) -> Self {
        self.root_instance_name = Some(value);
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

impl Default for CreateAppRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CreateAppRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateAppRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateAppRequestDataAttributesVisitor {
            type Value = CreateAppRequestDataAttributes;

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
                let mut name: Option<String> = None;
                let mut root_instance_name: Option<String> = None;
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

                let content = CreateAppRequestDataAttributes {
                    components,
                    description,
                    embedded_queries,
                    name,
                    root_instance_name,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateAppRequestDataAttributesVisitor)
    }
}
