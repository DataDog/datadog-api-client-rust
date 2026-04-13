// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a newly created secure embed shared dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecureEmbedCreateResponseAttributes {
    /// Creation timestamp.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The secret credential used for HMAC signing. Returned only on creation. Store securely — it cannot be retrieved again.
    #[serde(rename = "credential")]
    pub credential: Option<String>,
    /// The source dashboard ID.
    #[serde(rename = "dashboard_id")]
    pub dashboard_id: Option<String>,
    /// Default time range configuration for the secure embed.
    #[serde(rename = "global_time")]
    pub global_time: Option<crate::datadogV2::model::SecureEmbedGlobalTime>,
    /// Whether time range is viewer-selectable.
    #[serde(rename = "global_time_selectable")]
    pub global_time_selectable: Option<bool>,
    /// Internal share ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Template variables with their configuration.
    #[serde(rename = "selectable_template_vars")]
    pub selectable_template_vars:
        Option<Vec<crate::datadogV2::model::SecureEmbedSelectableTemplateVariable>>,
    /// The type of share. Always `secure_embed`.
    #[serde(rename = "share_type")]
    pub share_type: Option<crate::datadogV2::model::SecureEmbedShareType>,
    /// The status of the secure embed share. Active means the shared dashboard is available. Paused means it is not.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::SecureEmbedStatus>,
    /// Display title.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Public share token.
    #[serde(rename = "token")]
    pub token: Option<String>,
    /// CDN URL for the shared dashboard.
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Display settings for the secure embed shared dashboard.
    #[serde(rename = "viewing_preferences")]
    pub viewing_preferences: Option<crate::datadogV2::model::SecureEmbedViewingPreferences>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecureEmbedCreateResponseAttributes {
    pub fn new() -> SecureEmbedCreateResponseAttributes {
        SecureEmbedCreateResponseAttributes {
            created_at: None,
            credential: None,
            dashboard_id: None,
            global_time: None,
            global_time_selectable: None,
            id: None,
            selectable_template_vars: None,
            share_type: None,
            status: None,
            title: None,
            token: None,
            url: None,
            viewing_preferences: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn credential(mut self, value: String) -> Self {
        self.credential = Some(value);
        self
    }

    pub fn dashboard_id(mut self, value: String) -> Self {
        self.dashboard_id = Some(value);
        self
    }

    pub fn global_time(mut self, value: crate::datadogV2::model::SecureEmbedGlobalTime) -> Self {
        self.global_time = Some(value);
        self
    }

    pub fn global_time_selectable(mut self, value: bool) -> Self {
        self.global_time_selectable = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn selectable_template_vars(
        mut self,
        value: Vec<crate::datadogV2::model::SecureEmbedSelectableTemplateVariable>,
    ) -> Self {
        self.selectable_template_vars = Some(value);
        self
    }

    pub fn share_type(mut self, value: crate::datadogV2::model::SecureEmbedShareType) -> Self {
        self.share_type = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::SecureEmbedStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn token(mut self, value: String) -> Self {
        self.token = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }

    pub fn viewing_preferences(
        mut self,
        value: crate::datadogV2::model::SecureEmbedViewingPreferences,
    ) -> Self {
        self.viewing_preferences = Some(value);
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

impl Default for SecureEmbedCreateResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecureEmbedCreateResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecureEmbedCreateResponseAttributesVisitor;
        impl<'a> Visitor<'a> for SecureEmbedCreateResponseAttributesVisitor {
            type Value = SecureEmbedCreateResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut credential: Option<String> = None;
                let mut dashboard_id: Option<String> = None;
                let mut global_time: Option<crate::datadogV2::model::SecureEmbedGlobalTime> = None;
                let mut global_time_selectable: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut selectable_template_vars: Option<
                    Vec<crate::datadogV2::model::SecureEmbedSelectableTemplateVariable>,
                > = None;
                let mut share_type: Option<crate::datadogV2::model::SecureEmbedShareType> = None;
                let mut status: Option<crate::datadogV2::model::SecureEmbedStatus> = None;
                let mut title: Option<String> = None;
                let mut token: Option<String> = None;
                let mut url: Option<String> = None;
                let mut viewing_preferences: Option<
                    crate::datadogV2::model::SecureEmbedViewingPreferences,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "credential" => {
                            if v.is_null() {
                                continue;
                            }
                            credential = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dashboard_id" => {
                            if v.is_null() {
                                continue;
                            }
                            dashboard_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_time" => {
                            if v.is_null() {
                                continue;
                            }
                            global_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_time_selectable" => {
                            if v.is_null() {
                                continue;
                            }
                            global_time_selectable =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selectable_template_vars" => {
                            if v.is_null() {
                                continue;
                            }
                            selectable_template_vars =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "share_type" => {
                            if v.is_null() {
                                continue;
                            }
                            share_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _share_type) = share_type {
                                match _share_type {
                                    crate::datadogV2::model::SecureEmbedShareType::UnparsedObject(_share_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::SecureEmbedStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "token" => {
                            if v.is_null() {
                                continue;
                            }
                            token = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "viewing_preferences" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = SecureEmbedCreateResponseAttributes {
                    created_at,
                    credential,
                    dashboard_id,
                    global_time,
                    global_time_selectable,
                    id,
                    selectable_template_vars,
                    share_type,
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

        deserializer.deserialize_any(SecureEmbedCreateResponseAttributesVisitor)
    }
}
