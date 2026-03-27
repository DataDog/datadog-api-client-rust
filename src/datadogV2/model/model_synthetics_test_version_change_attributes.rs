// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a version change record.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestVersionChangeAttributes {
    /// UUID of the user who created this version.
    #[serde(rename = "author_uuid")]
    pub author_uuid: Option<String>,
    /// List of metadata describing individual changes in this version.
    #[serde(rename = "change_metadata")]
    pub change_metadata:
        Option<Vec<crate::datadogV2::model::SyntheticsTestVersionChangeMetadataItem>>,
    /// The sequential version number.
    #[serde(rename = "version_number")]
    pub version_number: Option<i64>,
    /// Timestamp of when this version was created.
    #[serde(rename = "version_payload_created_at")]
    pub version_payload_created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestVersionChangeAttributes {
    pub fn new() -> SyntheticsTestVersionChangeAttributes {
        SyntheticsTestVersionChangeAttributes {
            author_uuid: None,
            change_metadata: None,
            version_number: None,
            version_payload_created_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author_uuid(mut self, value: String) -> Self {
        self.author_uuid = Some(value);
        self
    }

    pub fn change_metadata(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestVersionChangeMetadataItem>,
    ) -> Self {
        self.change_metadata = Some(value);
        self
    }

    pub fn version_number(mut self, value: i64) -> Self {
        self.version_number = Some(value);
        self
    }

    pub fn version_payload_created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.version_payload_created_at = Some(value);
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

impl Default for SyntheticsTestVersionChangeAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestVersionChangeAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestVersionChangeAttributesVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestVersionChangeAttributesVisitor {
            type Value = SyntheticsTestVersionChangeAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author_uuid: Option<String> = None;
                let mut change_metadata: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestVersionChangeMetadataItem>,
                > = None;
                let mut version_number: Option<i64> = None;
                let mut version_payload_created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            author_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "change_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            change_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version_number" => {
                            if v.is_null() {
                                continue;
                            }
                            version_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version_payload_created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            version_payload_created_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestVersionChangeAttributes {
                    author_uuid,
                    change_metadata,
                    version_number,
                    version_payload_created_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestVersionChangeAttributesVisitor)
    }
}
