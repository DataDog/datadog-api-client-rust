// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing information about multiple users.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsersResponse {
    /// Array of returned users.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::User>>,
    /// Array of objects related to the users.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::UserResponseIncludedItem>>,
    /// Object describing meta attributes of response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ResponseMetaAttributes>,
}

impl UsersResponse {
    pub fn new() -> UsersResponse {
        UsersResponse {
            data: None,
            included: None,
            meta: None,
        }
    }

    pub fn data(mut self, value: Vec<crate::datadogV2::model::User>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn included(
        mut self,
        value: Vec<crate::datadogV2::model::UserResponseIncludedItem>,
    ) -> Self {
        self.included = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::ResponseMetaAttributes) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for UsersResponse {
    fn default() -> Self {
        Self::new()
    }
}
