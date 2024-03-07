// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GetTeamMembershipsSort {
    #[serde(rename = "manager_name")]
    MANAGER_NAME,
    #[serde(rename = "-manager_name")]
    _MANAGER_NAME,
    #[serde(rename = "name")]
    NAME,
    #[serde(rename = "-name")]
    _NAME,
    #[serde(rename = "handle")]
    HANDLE,
    #[serde(rename = "-handle")]
    _HANDLE,
    #[serde(rename = "email")]
    EMAIL,
    #[serde(rename = "-email")]
    _EMAIL,
}
impl ToString for GetTeamMembershipsSort {
    fn to_string(&self) -> String {
        match self {
            Self::MANAGER_NAME => String::from("manager_name"),
            Self::_MANAGER_NAME => String::from("-manager_name"),
            Self::NAME => String::from("name"),
            Self::_NAME => String::from("-name"),
            Self::HANDLE => String::from("handle"),
            Self::_HANDLE => String::from("-handle"),
            Self::EMAIL => String::from("email"),
            Self::_EMAIL => String::from("-email"),
        }
    }
}
