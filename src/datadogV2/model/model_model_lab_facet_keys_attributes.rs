// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Available facet key names for filtering resources.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModelLabFacetKeysAttributes {
    /// The list of available metric facet keys.
    #[serialize_always]
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<String>>,
    /// The list of available parameter facet keys.
    #[serde(rename = "parameters")]
    pub parameters: Vec<String>,
    /// The list of available tag facet keys.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ModelLabFacetKeysAttributes {
    pub fn new(
        metrics: Option<Vec<String>>,
        parameters: Vec<String>,
        tags: Vec<String>,
    ) -> ModelLabFacetKeysAttributes {
        ModelLabFacetKeysAttributes {
            metrics,
            parameters,
            tags,
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

impl<'de> Deserialize<'de> for ModelLabFacetKeysAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ModelLabFacetKeysAttributesVisitor;
        impl<'a> Visitor<'a> for ModelLabFacetKeysAttributesVisitor {
            type Value = ModelLabFacetKeysAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metrics: Option<Option<Vec<String>>> = None;
                let mut parameters: Option<Vec<String>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metrics" => {
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parameters" => {
                            parameters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let metrics = metrics.ok_or_else(|| M::Error::missing_field("metrics"))?;
                let parameters = parameters.ok_or_else(|| M::Error::missing_field("parameters"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = ModelLabFacetKeysAttributes {
                    metrics,
                    parameters,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ModelLabFacetKeysAttributesVisitor)
    }
}
