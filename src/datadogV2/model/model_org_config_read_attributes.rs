// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Readable attributes of an Org Config.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgConfigReadAttributes {
    /// The description of an Org Config.
    #[serde(rename = "description")]
    pub description: String,
    /// The timestamp of the last Org Config update (if any).
    #[serde(
        rename = "modified_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub modified_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The machine-friendly name of an Org Config.
    #[serde(rename = "name")]
    pub name: String,
    /// The value of an Org Config.
    #[serde(rename = "value")]
    pub value: serde_json::Value,
    /// The type of an Org Config value.
    #[serde(rename = "value_type")]
    pub value_type: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgConfigReadAttributes {
    pub fn new(
        description: String,
        name: String,
        value: serde_json::Value,
        value_type: String,
    ) -> OrgConfigReadAttributes {
        OrgConfigReadAttributes {
            description,
            modified_at: None,
            name,
            value,
            value_type,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn modified_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.modified_at = Some(value);
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

impl<'de> Deserialize<'de> for OrgConfigReadAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgConfigReadAttributesVisitor;
        impl<'a> Visitor<'a> for OrgConfigReadAttributesVisitor {
            type Value = OrgConfigReadAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut modified_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut name: Option<String> = None;
                let mut value: Option<serde_json::Value> = None;
                let mut value_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value_type" => {
                            value_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;
                let value_type = value_type.ok_or_else(|| M::Error::missing_field("value_type"))?;

                let content = OrgConfigReadAttributes {
                    description,
                    modified_at,
                    name,
                    value,
                    value_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgConfigReadAttributesVisitor)
    }
}
