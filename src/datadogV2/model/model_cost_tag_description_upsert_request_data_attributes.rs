// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Mutable attributes set when creating or updating a Cloud Cost Management tag key description.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostTagDescriptionUpsertRequestDataAttributes {
    /// Cloud provider this description applies to (for example, `aws`). Omit to set the cross-cloud default for the tag key.
    #[serde(rename = "cloud")]
    pub cloud: Option<String>,
    /// The human-readable description for the tag key.
    #[serde(rename = "description")]
    pub description: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostTagDescriptionUpsertRequestDataAttributes {
    pub fn new(description: String) -> CostTagDescriptionUpsertRequestDataAttributes {
        CostTagDescriptionUpsertRequestDataAttributes {
            cloud: None,
            description,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cloud(mut self, value: String) -> Self {
        self.cloud = Some(value);
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

impl<'de> Deserialize<'de> for CostTagDescriptionUpsertRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostTagDescriptionUpsertRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CostTagDescriptionUpsertRequestDataAttributesVisitor {
            type Value = CostTagDescriptionUpsertRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cloud: Option<String> = None;
                let mut description: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cloud" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = CostTagDescriptionUpsertRequestDataAttributes {
                    cloud,
                    description,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostTagDescriptionUpsertRequestDataAttributesVisitor)
    }
}
