// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability dataset version.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDatasetVersionDataAttributes {
    /// Unique identifier of the dataset this version belongs to.
    #[serde(rename = "dataset_id")]
    pub dataset_id: String,
    /// Timestamp when this dataset version was last referenced. Null if the version has never been used.
    #[serialize_always]
    #[serde(rename = "last_used")]
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    /// Sequential version number for this dataset version.
    #[serde(rename = "version_number")]
    pub version_number: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDatasetVersionDataAttributes {
    pub fn new(
        dataset_id: String,
        last_used: Option<chrono::DateTime<chrono::Utc>>,
        version_number: i32,
    ) -> LLMObsDatasetVersionDataAttributes {
        LLMObsDatasetVersionDataAttributes {
            dataset_id,
            last_used,
            version_number,
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

impl<'de> Deserialize<'de> for LLMObsDatasetVersionDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDatasetVersionDataAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsDatasetVersionDataAttributesVisitor {
            type Value = LLMObsDatasetVersionDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dataset_id: Option<String> = None;
                let mut last_used: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut version_number: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dataset_id" => {
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_used" => {
                            last_used = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version_number" => {
                            version_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let dataset_id = dataset_id.ok_or_else(|| M::Error::missing_field("dataset_id"))?;
                let last_used = last_used.ok_or_else(|| M::Error::missing_field("last_used"))?;
                let version_number =
                    version_number.ok_or_else(|| M::Error::missing_field("version_number"))?;

                let content = LLMObsDatasetVersionDataAttributes {
                    dataset_id,
                    last_used,
                    version_number,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDatasetVersionDataAttributesVisitor)
    }
}
