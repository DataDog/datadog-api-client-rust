// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team permission setting update
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamPermissionSettingUpdate {
    /// Team permission setting update attributes
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::TeamPermissionSettingUpdateAttributes>>,
    /// Team permission setting type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TeamPermissionSettingType,
}

impl TeamPermissionSettingUpdate {
    pub fn new(
        type_: crate::datadogV2::model::TeamPermissionSettingType,
    ) -> TeamPermissionSettingUpdate {
        TeamPermissionSettingUpdate {
            attributes: None,
            type_,
        }
    }
}
