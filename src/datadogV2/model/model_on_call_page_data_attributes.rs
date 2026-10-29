// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of an On-Call Page.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnCallPageDataAttributes {
    /// The time when the Page was first acknowledged.
    #[serde(rename = "acknowledged_at")]
    pub acknowledged_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The time when the Page was resolved.
    #[serde(rename = "closed_at")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The number of comments on the Page.
    #[serde(rename = "comment_count")]
    pub comment_count: Option<i64>,
    /// The time when the Page was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// A short summary of the issue or context.
    #[serde(rename = "description")]
    pub description: String,
    /// The ID of the escalation policy used to route the Page.
    #[serde(rename = "escalation_policy_uuid")]
    pub escalation_policy_uuid: Option<uuid::Uuid>,
    /// An alternative numeric identifier of the Page, as displayed in the Datadog UI.
    #[serde(rename = "key")]
    pub key: String,
    /// Live call information for a Page triggered by a live call route.
    #[serde(rename = "live_call")]
    pub live_call: Option<crate::datadogV2::model::OnCallPageLiveCall>,
    /// The time when the Page was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The time when the snooze ends. Only set while the Page is snoozed, together with `release_origin`.
    #[serde(rename = "release_at")]
    pub release_at: Option<chrono::DateTime<chrono::Utc>>,
    /// What snoozed the Page. Only set while the Page is snoozed, together with `release_at`.
    /// `ack_timeout`: the acknowledgment timeout. `team_rules_active_hours`: the Page was
    /// triggered outside of the active hours of the team's routing rules. `manual_snooze`:
    /// a user snoozed the Page.
    #[serde(rename = "release_origin")]
    pub release_origin: Option<crate::datadogV2::model::OnCallPageReleaseOrigin>,
    /// The time when responders were first added to the Page.
    #[serde(rename = "responders_added_at")]
    pub responders_added_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The services associated with the Page.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    /// The current status of the On-Call Page.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::OnCallPageStatus,
    /// Tags associated with the Page.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// The title of the Page.
    #[serde(rename = "title")]
    pub title: String,
    /// On-Call Page urgency level.
    #[serde(rename = "urgency")]
    pub urgency: crate::datadogV2::model::PageUrgency,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnCallPageDataAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        description: String,
        key: String,
        status: crate::datadogV2::model::OnCallPageStatus,
        tags: Vec<String>,
        title: String,
        urgency: crate::datadogV2::model::PageUrgency,
    ) -> OnCallPageDataAttributes {
        OnCallPageDataAttributes {
            acknowledged_at: None,
            closed_at: None,
            comment_count: None,
            created_at,
            description,
            escalation_policy_uuid: None,
            key,
            live_call: None,
            modified_at: None,
            release_at: None,
            release_origin: None,
            responders_added_at: None,
            services: None,
            status,
            tags,
            title,
            urgency,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn acknowledged_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.acknowledged_at = Some(value);
        self
    }

    pub fn closed_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.closed_at = Some(value);
        self
    }

    pub fn comment_count(mut self, value: i64) -> Self {
        self.comment_count = Some(value);
        self
    }

    pub fn escalation_policy_uuid(mut self, value: uuid::Uuid) -> Self {
        self.escalation_policy_uuid = Some(value);
        self
    }

    pub fn live_call(mut self, value: crate::datadogV2::model::OnCallPageLiveCall) -> Self {
        self.live_call = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn release_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.release_at = Some(value);
        self
    }

    pub fn release_origin(
        mut self,
        value: crate::datadogV2::model::OnCallPageReleaseOrigin,
    ) -> Self {
        self.release_origin = Some(value);
        self
    }

    pub fn responders_added_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.responders_added_at = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<String>) -> Self {
        self.services = Some(value);
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

impl<'de> Deserialize<'de> for OnCallPageDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnCallPageDataAttributesVisitor;
        impl<'a> Visitor<'a> for OnCallPageDataAttributesVisitor {
            type Value = OnCallPageDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut acknowledged_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut closed_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut comment_count: Option<i64> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut escalation_policy_uuid: Option<uuid::Uuid> = None;
                let mut key: Option<String> = None;
                let mut live_call: Option<crate::datadogV2::model::OnCallPageLiveCall> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut release_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut release_origin: Option<crate::datadogV2::model::OnCallPageReleaseOrigin> =
                    None;
                let mut responders_added_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut services: Option<Vec<String>> = None;
                let mut status: Option<crate::datadogV2::model::OnCallPageStatus> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut title: Option<String> = None;
                let mut urgency: Option<crate::datadogV2::model::PageUrgency> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "acknowledged_at" => {
                            if v.is_null() {
                                continue;
                            }
                            acknowledged_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "closed_at" => {
                            if v.is_null() {
                                continue;
                            }
                            closed_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "comment_count" => {
                            if v.is_null() {
                                continue;
                            }
                            comment_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "escalation_policy_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            escalation_policy_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "live_call" => {
                            if v.is_null() {
                                continue;
                            }
                            live_call = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "release_at" => {
                            if v.is_null() {
                                continue;
                            }
                            release_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "release_origin" => {
                            if v.is_null() {
                                continue;
                            }
                            release_origin =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _release_origin) = release_origin {
                                match _release_origin {
                                    crate::datadogV2::model::OnCallPageReleaseOrigin::UnparsedObject(_release_origin) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "responders_added_at" => {
                            if v.is_null() {
                                continue;
                            }
                            responders_added_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            if v.is_null() {
                                continue;
                            }
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::OnCallPageStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "urgency" => {
                            urgency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _urgency) = urgency {
                                match _urgency {
                                    crate::datadogV2::model::PageUrgency::UnparsedObject(
                                        _urgency,
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
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let urgency = urgency.ok_or_else(|| M::Error::missing_field("urgency"))?;

                let content = OnCallPageDataAttributes {
                    acknowledged_at,
                    closed_at,
                    comment_count,
                    created_at,
                    description,
                    escalation_policy_uuid,
                    key,
                    live_call,
                    modified_at,
                    release_at,
                    release_origin,
                    responders_added_at,
                    services,
                    status,
                    tags,
                    title,
                    urgency,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnCallPageDataAttributesVisitor)
    }
}
