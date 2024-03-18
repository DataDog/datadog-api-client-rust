// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team permission setting attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamPermissionSettingAttributes {
    /// The identifier for the action
    #[serde(rename = "action")]
    pub action: Option<crate::datadogV2::model::TeamPermissionSettingSerializerAction>,
    /// Whether or not the permission setting is editable by the current user
    #[serde(rename = "editable")]
    pub editable: Option<bool>,
    /// Possible values for action
    #[serde(rename = "options")]
    pub options: Option<Vec<crate::datadogV2::model::TeamPermissionSettingValue>>,
    /// The team permission name
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// What type of user is allowed to perform the specified action
    #[serde(rename = "value")]
    pub value: Option<crate::datadogV2::model::TeamPermissionSettingValue>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamPermissionSettingAttributes {
    pub fn new() -> TeamPermissionSettingAttributes {
        TeamPermissionSettingAttributes {
            action: None,
            editable: None,
            options: None,
            title: None,
            value: None,
            _unparsed: false,
        }
    }

    pub fn action(
        mut self,
        value: crate::datadogV2::model::TeamPermissionSettingSerializerAction,
    ) -> Self {
        self.action = Some(value);
        self
    }

    pub fn editable(mut self, value: bool) -> Self {
        self.editable = Some(value);
        self
    }

    pub fn options(
        mut self,
        value: Vec<crate::datadogV2::model::TeamPermissionSettingValue>,
    ) -> Self {
        self.options = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn value(mut self, value: crate::datadogV2::model::TeamPermissionSettingValue) -> Self {
        self.value = Some(value);
        self
    }
}

impl Default for TeamPermissionSettingAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamPermissionSettingAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamPermissionSettingAttributesVisitor;
        impl<'a> Visitor<'a> for TeamPermissionSettingAttributesVisitor {
            type Value = TeamPermissionSettingAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<
                    crate::datadogV2::model::TeamPermissionSettingSerializerAction,
                > = None;
                let mut editable: Option<bool> = None;
                let mut options: Option<Vec<crate::datadogV2::model::TeamPermissionSettingValue>> =
                    None;
                let mut title: Option<String> = None;
                let mut value: Option<crate::datadogV2::model::TeamPermissionSettingValue> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            if v.is_null() {
                                continue;
                            }
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _action) = action {
                                match _action {
                                    crate::datadogV2::model::TeamPermissionSettingSerializerAction::UnparsedObject(_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "editable" => {
                            if v.is_null() {
                                continue;
                            }
                            editable = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _value) = value {
                                match _value {
                                    crate::datadogV2::model::TeamPermissionSettingValue::UnparsedObject(_value) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = TeamPermissionSettingAttributes {
                    action,
                    editable,
                    options,
                    title,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamPermissionSettingAttributesVisitor)
    }
}
