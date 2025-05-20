// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a resource filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResourceFilterAttributes {
    /// A map of cloud provider names (e.g., "aws", "gcp", "azure") to a map of account/resource IDs and their associated tag filters.
    #[serde(rename = "cloud_provider")]
    pub cloud_provider:
        std::collections::BTreeMap<String, std::collections::BTreeMap<String, Vec<String>>>,
    /// The UUID of the resource filter.
    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ResourceFilterAttributes {
    pub fn new(
        cloud_provider: std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, Vec<String>>,
        >,
    ) -> ResourceFilterAttributes {
        ResourceFilterAttributes {
            cloud_provider,
            uuid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
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

impl<'de> Deserialize<'de> for ResourceFilterAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ResourceFilterAttributesVisitor;
        impl<'a> Visitor<'a> for ResourceFilterAttributesVisitor {
            type Value = ResourceFilterAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cloud_provider: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<String, Vec<String>>,
                    >,
                > = None;
                let mut uuid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cloud_provider" => {
                            cloud_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cloud_provider =
                    cloud_provider.ok_or_else(|| M::Error::missing_field("cloud_provider"))?;

                let content = ResourceFilterAttributes {
                    cloud_provider,
                    uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ResourceFilterAttributesVisitor)
    }
}
