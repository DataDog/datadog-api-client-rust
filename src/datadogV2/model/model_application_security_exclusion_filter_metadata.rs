// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the exclusion filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityExclusionFilterMetadata {
    /// The timestamp when the exclusion filter was added.
    #[serde(rename = "added_at")]
    pub added_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The email address of the user who added the exclusion filter.
    #[serde(rename = "added_by")]
    pub added_by: Option<String>,
    /// The timestamp when the exclusion filter was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The email address of the user who last modified the exclusion filter.
    #[serde(rename = "modified_by")]
    pub modified_by: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityExclusionFilterMetadata {
    pub fn new() -> ApplicationSecurityExclusionFilterMetadata {
        ApplicationSecurityExclusionFilterMetadata {
            added_at: None,
            added_by: None,
            modified_at: None,
            modified_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn added_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.added_at = Some(value);
        self
    }

    pub fn added_by(mut self, value: String) -> Self {
        self.added_by = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn modified_by(mut self, value: String) -> Self {
        self.modified_by = Some(value);
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

impl Default for ApplicationSecurityExclusionFilterMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationSecurityExclusionFilterMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityExclusionFilterMetadataVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityExclusionFilterMetadataVisitor {
            type Value = ApplicationSecurityExclusionFilterMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut added_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut added_by: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "added_at" => {
                            if v.is_null() {
                                continue;
                            }
                            added_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "added_by" => {
                            if v.is_null() {
                                continue;
                            }
                            added_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ApplicationSecurityExclusionFilterMetadata {
                    added_at,
                    added_by,
                    modified_at,
                    modified_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityExclusionFilterMetadataVisitor)
    }
}
