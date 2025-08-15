// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `CreateAppsDatastoreRequestDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateAppsDatastoreRequestDataAttributes {
    /// The `attributes` `description`.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The `attributes` `name`.
    #[serde(rename = "name")]
    pub name: String,
    /// The `attributes` `org_access`.
    #[serde(rename = "org_access")]
    pub org_access: Option<String>,
    /// The `attributes` `primary_column_name`.
    #[serde(rename = "primary_column_name")]
    pub primary_column_name: String,
    /// The `attributes` `primary_key_generation_strategy`.
    #[serde(rename = "primary_key_generation_strategy")]
    pub primary_key_generation_strategy: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateAppsDatastoreRequestDataAttributes {
    pub fn new(
        name: String,
        primary_column_name: String,
    ) -> CreateAppsDatastoreRequestDataAttributes {
        CreateAppsDatastoreRequestDataAttributes {
            description: None,
            name,
            org_access: None,
            primary_column_name,
            primary_key_generation_strategy: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn org_access(mut self, value: String) -> Self {
        self.org_access = Some(value);
        self
    }

    pub fn primary_key_generation_strategy(mut self, value: String) -> Self {
        self.primary_key_generation_strategy = Some(value);
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

impl<'de> Deserialize<'de> for CreateAppsDatastoreRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateAppsDatastoreRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateAppsDatastoreRequestDataAttributesVisitor {
            type Value = CreateAppsDatastoreRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut org_access: Option<String> = None;
                let mut primary_column_name: Option<String> = None;
                let mut primary_key_generation_strategy: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_access" => {
                            if v.is_null() {
                                continue;
                            }
                            org_access = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_column_name" => {
                            primary_column_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_key_generation_strategy" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_key_generation_strategy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let primary_column_name = primary_column_name
                    .ok_or_else(|| M::Error::missing_field("primary_column_name"))?;

                let content = CreateAppsDatastoreRequestDataAttributes {
                    description,
                    name,
                    org_access,
                    primary_column_name,
                    primary_key_generation_strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateAppsDatastoreRequestDataAttributesVisitor)
    }
}
