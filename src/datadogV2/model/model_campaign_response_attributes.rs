// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Campaign attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CampaignResponseAttributes {
    /// Creation time of the campaign.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The description of the campaign.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The due date of the campaign.
    #[serde(rename = "due_date")]
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Entity scope query to filter entities for this campaign.
    #[serde(rename = "entity_scope")]
    pub entity_scope: Option<String>,
    /// Guidance for the campaign.
    #[serde(rename = "guidance")]
    pub guidance: Option<String>,
    /// The unique key for the campaign.
    #[serde(rename = "key")]
    pub key: String,
    /// Time of last campaign modification.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The name of the campaign.
    #[serde(rename = "name")]
    pub name: String,
    /// The UUID of the campaign owner.
    #[serde(rename = "owner")]
    pub owner: String,
    /// The start date of the campaign.
    #[serde(rename = "start_date")]
    pub start_date: chrono::DateTime<chrono::Utc>,
    /// The status of the campaign.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CampaignResponseAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        key: String,
        modified_at: chrono::DateTime<chrono::Utc>,
        name: String,
        owner: String,
        start_date: chrono::DateTime<chrono::Utc>,
        status: String,
    ) -> CampaignResponseAttributes {
        CampaignResponseAttributes {
            created_at,
            description: None,
            due_date: None,
            entity_scope: None,
            guidance: None,
            key,
            modified_at,
            name,
            owner,
            start_date,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn due_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn entity_scope(mut self, value: String) -> Self {
        self.entity_scope = Some(value);
        self
    }

    pub fn guidance(mut self, value: String) -> Self {
        self.guidance = Some(value);
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

impl<'de> Deserialize<'de> for CampaignResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CampaignResponseAttributesVisitor;
        impl<'a> Visitor<'a> for CampaignResponseAttributesVisitor {
            type Value = CampaignResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut due_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut entity_scope: Option<String> = None;
                let mut guidance: Option<String> = None;
                let mut key: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut owner: Option<String> = None;
                let mut start_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "due_date" => {
                            if v.is_null() {
                                continue;
                            }
                            due_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity_scope" => {
                            if v.is_null() {
                                continue;
                            }
                            entity_scope =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "guidance" => {
                            if v.is_null() {
                                continue;
                            }
                            guidance = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "owner" => {
                            owner = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_date" => {
                            start_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let owner = owner.ok_or_else(|| M::Error::missing_field("owner"))?;
                let start_date = start_date.ok_or_else(|| M::Error::missing_field("start_date"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = CampaignResponseAttributes {
                    created_at,
                    description,
                    due_date,
                    entity_scope,
                    guidance,
                    key,
                    modified_at,
                    name,
                    owner,
                    start_date,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CampaignResponseAttributesVisitor)
    }
}
