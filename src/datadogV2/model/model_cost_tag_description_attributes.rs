// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Human-readable description and metadata attached to a Cloud Cost Management tag key, optionally scoped to a single cloud provider.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostTagDescriptionAttributes {
    /// Cloud provider this description applies to (for example, `aws`). Empty when the description is the cross-cloud default for the tag key.
    #[serde(rename = "cloud")]
    pub cloud: String,
    /// Timestamp when the description was created, in RFC 3339 format.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The human-readable description for the tag key.
    #[serde(rename = "description")]
    pub description: String,
    /// Origin of the description. `human` indicates the description was written by a user, `ai_generated` was produced by AI, and `datadog` is a default supplied by Datadog.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::CostTagDescriptionSource,
    /// The tag key this description applies to.
    #[serde(rename = "tag_key")]
    pub tag_key: String,
    /// Timestamp when the description was last updated, in RFC 3339 format.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostTagDescriptionAttributes {
    pub fn new(
        cloud: String,
        created_at: String,
        description: String,
        source: crate::datadogV2::model::CostTagDescriptionSource,
        tag_key: String,
        updated_at: String,
    ) -> CostTagDescriptionAttributes {
        CostTagDescriptionAttributes {
            cloud,
            created_at,
            description,
            source,
            tag_key,
            updated_at,
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

impl<'de> Deserialize<'de> for CostTagDescriptionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostTagDescriptionAttributesVisitor;
        impl<'a> Visitor<'a> for CostTagDescriptionAttributesVisitor {
            type Value = CostTagDescriptionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cloud: Option<String> = None;
                let mut created_at: Option<String> = None;
                let mut description: Option<String> = None;
                let mut source: Option<crate::datadogV2::model::CostTagDescriptionSource> = None;
                let mut tag_key: Option<String> = None;
                let mut updated_at: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cloud" => {
                            cloud = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::CostTagDescriptionSource::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tag_key" => {
                            tag_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cloud = cloud.ok_or_else(|| M::Error::missing_field("cloud"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let tag_key = tag_key.ok_or_else(|| M::Error::missing_field("tag_key"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = CostTagDescriptionAttributes {
                    cloud,
                    created_at,
                    description,
                    source,
                    tag_key,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostTagDescriptionAttributesVisitor)
    }
}
