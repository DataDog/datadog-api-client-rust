// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a new campaign.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateCampaignRequestAttributes {
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
    /// The name of the campaign.
    #[serde(rename = "name")]
    pub name: String,
    /// The UUID of the campaign owner.
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    /// Array of rule IDs associated with this campaign.
    #[serde(rename = "rule_ids")]
    pub rule_ids: Vec<String>,
    /// The start date of the campaign.
    #[serde(rename = "start_date")]
    pub start_date: chrono::DateTime<chrono::Utc>,
    /// The status of the campaign.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::CampaignStatus>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateCampaignRequestAttributes {
    pub fn new(
        key: String,
        name: String,
        owner_id: String,
        rule_ids: Vec<String>,
        start_date: chrono::DateTime<chrono::Utc>,
    ) -> CreateCampaignRequestAttributes {
        CreateCampaignRequestAttributes {
            description: None,
            due_date: None,
            entity_scope: None,
            guidance: None,
            key,
            name,
            owner_id,
            rule_ids,
            start_date,
            status: None,
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

    pub fn status(mut self, value: crate::datadogV2::model::CampaignStatus) -> Self {
        self.status = Some(value);
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

impl<'de> Deserialize<'de> for CreateCampaignRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateCampaignRequestAttributesVisitor;
        impl<'a> Visitor<'a> for CreateCampaignRequestAttributesVisitor {
            type Value = CreateCampaignRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut due_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut entity_scope: Option<String> = None;
                let mut guidance: Option<String> = None;
                let mut key: Option<String> = None;
                let mut name: Option<String> = None;
                let mut owner_id: Option<String> = None;
                let mut rule_ids: Option<Vec<String>> = None;
                let mut start_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<crate::datadogV2::model::CampaignStatus> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "owner_id" => {
                            owner_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_ids" => {
                            rule_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_date" => {
                            start_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::CampaignStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let owner_id = owner_id.ok_or_else(|| M::Error::missing_field("owner_id"))?;
                let rule_ids = rule_ids.ok_or_else(|| M::Error::missing_field("rule_ids"))?;
                let start_date = start_date.ok_or_else(|| M::Error::missing_field("start_date"))?;

                let content = CreateCampaignRequestAttributes {
                    description,
                    due_date,
                    entity_scope,
                    guidance,
                    key,
                    name,
                    owner_id,
                    rule_ids,
                    start_date,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateCampaignRequestAttributesVisitor)
    }
}
