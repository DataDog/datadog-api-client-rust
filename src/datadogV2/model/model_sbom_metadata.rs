// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Provides additional information about a BOM.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SBOMMetadata {
    /// List of authors of the SBOM.
    #[serde(rename = "authors")]
    pub authors: Option<Vec<crate::datadogV2::model::SBOMMetadataAuthor>>,
    /// The component that the BOM describes.
    #[serde(rename = "component")]
    pub component: Option<crate::datadogV2::model::SBOMMetadataComponent>,
    /// The timestamp of the SBOM creation.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SBOMMetadata {
    pub fn new() -> SBOMMetadata {
        SBOMMetadata {
            authors: None,
            component: None,
            timestamp: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn authors(mut self, value: Vec<crate::datadogV2::model::SBOMMetadataAuthor>) -> Self {
        self.authors = Some(value);
        self
    }

    pub fn component(mut self, value: crate::datadogV2::model::SBOMMetadataComponent) -> Self {
        self.component = Some(value);
        self
    }

    pub fn timestamp(mut self, value: String) -> Self {
        self.timestamp = Some(value);
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

impl Default for SBOMMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SBOMMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SBOMMetadataVisitor;
        impl<'a> Visitor<'a> for SBOMMetadataVisitor {
            type Value = SBOMMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authors: Option<Vec<crate::datadogV2::model::SBOMMetadataAuthor>> = None;
                let mut component: Option<crate::datadogV2::model::SBOMMetadataComponent> = None;
                let mut timestamp: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authors" => {
                            if v.is_null() {
                                continue;
                            }
                            authors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "component" => {
                            if v.is_null() {
                                continue;
                            }
                            component = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SBOMMetadata {
                    authors,
                    component,
                    timestamp,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SBOMMetadataVisitor)
    }
}
