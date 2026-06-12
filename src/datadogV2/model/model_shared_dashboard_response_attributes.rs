// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a shared dashboard response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SharedDashboardResponseAttributes {
    /// Time when the shared dashboard was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Domains where embed-type shared dashboards can be embedded.
    #[serde(rename = "embeddable_domains")]
    pub embeddable_domains: Vec<String>,
    /// Time when the shared dashboard expires.
    #[serialize_always]
    #[serde(rename = "expiration")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    /// Default time range configuration for the shared dashboard.
    #[serialize_always]
    #[serde(rename = "global_time")]
    pub global_time: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Whether viewers can select a different global time setting.
    #[serde(rename = "global_time_selectable")]
    pub global_time_selectable: bool,
    /// Invitees for invite-only shared dashboards.
    #[serde(rename = "invitees")]
    pub invitees: Vec<crate::datadogV2::model::SharedDashboardInvitee>,
    /// Time when the shared dashboard was last accessed.
    #[serialize_always]
    #[serde(rename = "last_accessed")]
    pub last_accessed: Option<chrono::DateTime<chrono::Utc>>,
    /// Template variables that viewers can modify.
    #[serde(rename = "selectable_template_vars")]
    pub selectable_template_vars:
        Vec<crate::datadogV2::model::SharedDashboardSelectableTemplateVariable>,
    /// Type of dashboard sharing.
    #[serde(rename = "share_type")]
    pub share_type: crate::datadogV2::model::SharedDashboardShareType,
    /// Whether the user who shared the dashboard is disabled.
    #[serde(rename = "sharer_disabled")]
    pub sharer_disabled: bool,
    /// Status of the shared dashboard.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::SharedDashboardStatus,
    /// Display title for the shared dashboard.
    #[serde(rename = "title")]
    pub title: String,
    /// Token assigned to the shared dashboard.
    #[serde(rename = "token")]
    pub token: String,
    /// URL for the shared dashboard.
    #[serde(rename = "url")]
    pub url: String,
    /// Display settings for the shared dashboard.
    #[serde(rename = "viewing_preferences")]
    pub viewing_preferences: crate::datadogV2::model::SharedDashboardViewingPreferences,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SharedDashboardResponseAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        embeddable_domains: Vec<String>,
        expiration: Option<chrono::DateTime<chrono::Utc>>,
        global_time: Option<std::collections::BTreeMap<String, serde_json::Value>>,
        global_time_selectable: bool,
        invitees: Vec<crate::datadogV2::model::SharedDashboardInvitee>,
        last_accessed: Option<chrono::DateTime<chrono::Utc>>,
        selectable_template_vars: Vec<
            crate::datadogV2::model::SharedDashboardSelectableTemplateVariable,
        >,
        share_type: crate::datadogV2::model::SharedDashboardShareType,
        sharer_disabled: bool,
        status: crate::datadogV2::model::SharedDashboardStatus,
        title: String,
        token: String,
        url: String,
        viewing_preferences: crate::datadogV2::model::SharedDashboardViewingPreferences,
    ) -> SharedDashboardResponseAttributes {
        SharedDashboardResponseAttributes {
            created_at,
            embeddable_domains,
            expiration,
            global_time,
            global_time_selectable,
            invitees,
            last_accessed,
            selectable_template_vars,
            share_type,
            sharer_disabled,
            status,
            title,
            token,
            url,
            viewing_preferences,
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

impl<'de> Deserialize<'de> for SharedDashboardResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SharedDashboardResponseAttributesVisitor;
        impl<'a> Visitor<'a> for SharedDashboardResponseAttributesVisitor {
            type Value = SharedDashboardResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut embeddable_domains: Option<Vec<String>> = None;
                let mut expiration: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut global_time: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut global_time_selectable: Option<bool> = None;
                let mut invitees: Option<Vec<crate::datadogV2::model::SharedDashboardInvitee>> =
                    None;
                let mut last_accessed: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut selectable_template_vars: Option<
                    Vec<crate::datadogV2::model::SharedDashboardSelectableTemplateVariable>,
                > = None;
                let mut share_type: Option<crate::datadogV2::model::SharedDashboardShareType> =
                    None;
                let mut sharer_disabled: Option<bool> = None;
                let mut status: Option<crate::datadogV2::model::SharedDashboardStatus> = None;
                let mut title: Option<String> = None;
                let mut token: Option<String> = None;
                let mut url: Option<String> = None;
                let mut viewing_preferences: Option<
                    crate::datadogV2::model::SharedDashboardViewingPreferences,
                > = None;
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
                        "embeddable_domains" => {
                            embeddable_domains =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expiration" => {
                            expiration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_time" => {
                            global_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_time_selectable" => {
                            global_time_selectable =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "invitees" => {
                            invitees = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_accessed" => {
                            last_accessed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selectable_template_vars" => {
                            selectable_template_vars =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "share_type" => {
                            share_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _share_type) = share_type {
                                match _share_type {
                                    crate::datadogV2::model::SharedDashboardShareType::UnparsedObject(_share_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "sharer_disabled" => {
                            sharer_disabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::SharedDashboardStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "token" => {
                            token = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "viewing_preferences" => {
                            viewing_preferences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let embeddable_domains = embeddable_domains
                    .ok_or_else(|| M::Error::missing_field("embeddable_domains"))?;
                let expiration = expiration.ok_or_else(|| M::Error::missing_field("expiration"))?;
                let global_time =
                    global_time.ok_or_else(|| M::Error::missing_field("global_time"))?;
                let global_time_selectable = global_time_selectable
                    .ok_or_else(|| M::Error::missing_field("global_time_selectable"))?;
                let invitees = invitees.ok_or_else(|| M::Error::missing_field("invitees"))?;
                let last_accessed =
                    last_accessed.ok_or_else(|| M::Error::missing_field("last_accessed"))?;
                let selectable_template_vars = selectable_template_vars
                    .ok_or_else(|| M::Error::missing_field("selectable_template_vars"))?;
                let share_type = share_type.ok_or_else(|| M::Error::missing_field("share_type"))?;
                let sharer_disabled =
                    sharer_disabled.ok_or_else(|| M::Error::missing_field("sharer_disabled"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let token = token.ok_or_else(|| M::Error::missing_field("token"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;
                let viewing_preferences = viewing_preferences
                    .ok_or_else(|| M::Error::missing_field("viewing_preferences"))?;

                let content = SharedDashboardResponseAttributes {
                    created_at,
                    embeddable_domains,
                    expiration,
                    global_time,
                    global_time_selectable,
                    invitees,
                    last_accessed,
                    selectable_template_vars,
                    share_type,
                    sharer_disabled,
                    status,
                    title,
                    token,
                    url,
                    viewing_preferences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SharedDashboardResponseAttributesVisitor)
    }
}
