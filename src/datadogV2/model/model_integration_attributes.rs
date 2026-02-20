// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationAttributes {
    /// List of categories associated with the integration.
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
    /// A description of the integration.
    #[serde(rename = "description")]
    pub description: String,
    /// Whether the integration is installed.
    #[serde(rename = "installed")]
    pub installed: bool,
    /// The name of the integration.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationAttributes {
    pub fn new(
        categories: Vec<String>,
        description: String,
        installed: bool,
        title: String,
    ) -> IntegrationAttributes {
        IntegrationAttributes {
            categories,
            description,
            installed,
            title,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for IntegrationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationAttributesVisitor;
        impl<'a> Visitor<'a> for IntegrationAttributesVisitor {
            type Value = IntegrationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut categories: Option<Vec<String>> = None;
                let mut description: Option<String> = None;
                let mut installed: Option<bool> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "categories" => {
                            categories = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "installed" => {
                            installed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let categories = categories.ok_or_else(|| M::Error::missing_field("categories"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let installed = installed.ok_or_else(|| M::Error::missing_field("installed"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = IntegrationAttributes {
                    categories,
                    description,
                    installed,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationAttributesVisitor)
    }
}
