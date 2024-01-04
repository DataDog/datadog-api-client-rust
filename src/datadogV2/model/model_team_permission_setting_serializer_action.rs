// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TeamPermissionSettingSerializerAction {
    #[serde(rename = "manage_membership")]
    MANAGE_MEMBERSHIP,
    #[serde(rename = "edit")]
    EDIT,
}

impl ToString for TeamPermissionSettingSerializerAction {
    fn to_string(&self) -> String {
        match self {
            Self::MANAGE_MEMBERSHIP => String::from("manage_membership"),
            Self::EDIT => String::from("edit"),
        }
    }
}
