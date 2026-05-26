// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata associated with the WAF policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityPolicyMetadata {
    /// The date and time the WAF policy was created.
    #[serde(rename = "added_at")]
    pub added_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The handle of the user who created the WAF policy.
    #[serde(rename = "added_by")]
    pub added_by: Option<String>,
    /// The name of the user who created the WAF policy.
    #[serde(rename = "added_by_name")]
    pub added_by_name: Option<String>,
    /// The date and time the WAF policy was last updated.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The handle of the user who last updated the WAF policy.
    #[serde(rename = "modified_by")]
    pub modified_by: Option<String>,
    /// The name of the user who last updated the WAF policy.
    #[serde(rename = "modified_by_name")]
    pub modified_by_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityPolicyMetadata {
    pub fn new() -> ApplicationSecurityPolicyMetadata {
        ApplicationSecurityPolicyMetadata {
            added_at: None,
            added_by: None,
            added_by_name: None,
            modified_at: None,
            modified_by: None,
            modified_by_name: None,
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

    pub fn added_by_name(mut self, value: String) -> Self {
        self.added_by_name = Some(value);
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

    pub fn modified_by_name(mut self, value: String) -> Self {
        self.modified_by_name = Some(value);
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

impl Default for ApplicationSecurityPolicyMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationSecurityPolicyMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityPolicyMetadataVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityPolicyMetadataVisitor {
            type Value = ApplicationSecurityPolicyMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut added_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut added_by: Option<String> = None;
                let mut added_by_name: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<String> = None;
                let mut modified_by_name: Option<String> = None;
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
                        "added_by_name" => {
                            if v.is_null() {
                                continue;
                            }
                            added_by_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "modified_by_name" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_by_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ApplicationSecurityPolicyMetadata {
                    added_at,
                    added_by,
                    added_by_name,
                    modified_at,
                    modified_by,
                    modified_by_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityPolicyMetadataVisitor)
    }
}
