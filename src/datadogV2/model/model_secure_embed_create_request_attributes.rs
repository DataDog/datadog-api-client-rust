// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a secure embed shared dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecureEmbedCreateRequestAttributes {
    /// Default time range configuration for the secure embed.
    #[serde(rename = "global_time")]
    pub global_time: crate::datadogV2::model::SecureEmbedGlobalTime,
    /// Whether viewers can change the time range.
    #[serde(rename = "global_time_selectable")]
    pub global_time_selectable: bool,
    /// Template variables viewers can modify.
    #[serde(rename = "selectable_template_vars")]
    pub selectable_template_vars:
        Vec<crate::datadogV2::model::SecureEmbedSelectableTemplateVariable>,
    /// The status of the secure embed share. Active means the shared dashboard is available. Paused means it is not.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::SecureEmbedStatus,
    /// Display title for the shared dashboard.
    #[serde(rename = "title")]
    pub title: String,
    /// Display settings for the secure embed shared dashboard.
    #[serde(rename = "viewing_preferences")]
    pub viewing_preferences: crate::datadogV2::model::SecureEmbedViewingPreferences,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecureEmbedCreateRequestAttributes {
    pub fn new(
        global_time: crate::datadogV2::model::SecureEmbedGlobalTime,
        global_time_selectable: bool,
        selectable_template_vars: Vec<
            crate::datadogV2::model::SecureEmbedSelectableTemplateVariable,
        >,
        status: crate::datadogV2::model::SecureEmbedStatus,
        title: String,
        viewing_preferences: crate::datadogV2::model::SecureEmbedViewingPreferences,
    ) -> SecureEmbedCreateRequestAttributes {
        SecureEmbedCreateRequestAttributes {
            global_time,
            global_time_selectable,
            selectable_template_vars,
            status,
            title,
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

impl<'de> Deserialize<'de> for SecureEmbedCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecureEmbedCreateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for SecureEmbedCreateRequestAttributesVisitor {
            type Value = SecureEmbedCreateRequestAttributes;

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
                            global_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_time_selectable" => {
                            global_time_selectable =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selectable_template_vars" => {
                            selectable_template_vars =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
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
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let global_time =
                    global_time.ok_or_else(|| M::Error::missing_field("global_time"))?;
                let global_time_selectable = global_time_selectable
                    .ok_or_else(|| M::Error::missing_field("global_time_selectable"))?;
                let selectable_template_vars = selectable_template_vars
                    .ok_or_else(|| M::Error::missing_field("selectable_template_vars"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let viewing_preferences = viewing_preferences
                    .ok_or_else(|| M::Error::missing_field("viewing_preferences"))?;

                let content = SecureEmbedCreateRequestAttributes {
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

        deserializer.deserialize_any(SecureEmbedCreateRequestAttributesVisitor)
    }
}
