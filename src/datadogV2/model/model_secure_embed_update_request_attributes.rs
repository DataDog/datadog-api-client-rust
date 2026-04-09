// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating a secure embed shared dashboard. All fields are optional.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecureEmbedUpdateRequestAttributes {
    /// Default time range configuration for the secure embed.
    #[serde(rename = "global_time")]
    pub global_time: Option<crate::datadogV2::model::SecureEmbedGlobalTime>,
    /// Updated time selectability.
    #[serde(rename = "global_time_selectable")]
    pub global_time_selectable: Option<bool>,
    /// Updated template variables.
    #[serde(rename = "selectable_template_vars")]
    pub selectable_template_vars:
        Option<Vec<crate::datadogV2::model::SecureEmbedSelectableTemplateVariable>>,
    /// The status of the secure embed share. Active means the shared dashboard is available. Paused means it is not.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::SecureEmbedStatus>,
    /// Updated title.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Display settings for the secure embed shared dashboard.
    #[serde(rename = "viewing_preferences")]
    pub viewing_preferences: Option<crate::datadogV2::model::SecureEmbedViewingPreferences>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecureEmbedUpdateRequestAttributes {
    pub fn new() -> SecureEmbedUpdateRequestAttributes {
        SecureEmbedUpdateRequestAttributes {
            global_time: None,
            global_time_selectable: None,
            selectable_template_vars: None,
            status: None,
            title: None,
            viewing_preferences: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn global_time(mut self, value: crate::datadogV2::model::SecureEmbedGlobalTime) -> Self {
        self.global_time = Some(value);
        self
    }

    pub fn global_time_selectable(mut self, value: bool) -> Self {
        self.global_time_selectable = Some(value);
        self
    }

    pub fn selectable_template_vars(
        mut self,
        value: Vec<crate::datadogV2::model::SecureEmbedSelectableTemplateVariable>,
    ) -> Self {
        self.selectable_template_vars = Some(value);
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

impl Default for SecureEmbedUpdateRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecureEmbedUpdateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecureEmbedUpdateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for SecureEmbedUpdateRequestAttributesVisitor {
            type Value = SecureEmbedUpdateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut global_time: Option<crate::datadogV2::model::SecureEmbedGlobalTime> = None;
                let mut global_time_selectable: Option<bool> = None;
                let mut selectable_template_vars: Option<
                    Vec<crate::datadogV2::model::SecureEmbedSelectableTemplateVariable>,
                > = None;
                let mut status: Option<crate::datadogV2::model::SecureEmbedStatus> = None;
                let mut title: Option<String> = None;
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
                        "selectable_template_vars" => {
                            if v.is_null() {
                                continue;
                            }
                            selectable_template_vars =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = SecureEmbedUpdateRequestAttributes {
                    global_time,
                    global_time_selectable,
                    selectable_template_vars,
                    status,
                    title,
                    viewing_preferences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecureEmbedUpdateRequestAttributesVisitor)
    }
}
