// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team permission setting attributes
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
}

impl TeamPermissionSettingAttributes {
    pub fn new() -> TeamPermissionSettingAttributes {
        TeamPermissionSettingAttributes {
            action: None,
            editable: None,
            options: None,
            title: None,
            value: None,
        }
    }
}
