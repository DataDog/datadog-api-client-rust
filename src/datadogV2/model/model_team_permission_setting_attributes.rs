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

    pub fn action(
        &mut self,
        value: crate::datadogV2::model::TeamPermissionSettingSerializerAction,
    ) -> &mut Self {
        self.action = Some(value);
        self
    }

    pub fn editable(&mut self, value: bool) -> &mut Self {
        self.editable = Some(value);
        self
    }

    pub fn options(
        &mut self,
        value: Vec<crate::datadogV2::model::TeamPermissionSettingValue>,
    ) -> &mut Self {
        self.options = Some(value);
        self
    }

    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }

    pub fn value(
        &mut self,
        value: crate::datadogV2::model::TeamPermissionSettingValue,
    ) -> &mut Self {
        self.value = Some(value);
        self
    }
}

impl Default for TeamPermissionSettingAttributes {
    fn default() -> Self {
        Self::new()
    }
}
