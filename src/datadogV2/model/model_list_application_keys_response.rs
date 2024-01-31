// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response for a list of application keys.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListApplicationKeysResponse {
    /// Array of application keys.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::PartialApplicationKey>>,
    /// Array of objects related to the application key.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::ApplicationKeyResponseIncludedItem>>,
    /// Additional information related to the application key response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ApplicationKeyResponseMeta>,
}

impl ListApplicationKeysResponse {
    pub fn new() -> ListApplicationKeysResponse {
        ListApplicationKeysResponse {
            data: None,
            included: None,
            meta: None,
        }
    }

    pub fn with_data(
        &mut self,
        value: Vec<crate::datadogV2::model::PartialApplicationKey>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn with_included(
        &mut self,
        value: Vec<crate::datadogV2::model::ApplicationKeyResponseIncludedItem>,
    ) -> &mut Self {
        self.included = Some(value);
        self
    }

    pub fn with_meta(
        &mut self,
        value: crate::datadogV2::model::ApplicationKeyResponseMeta,
    ) -> &mut Self {
        self.meta = Some(value);
        self
    }
}
impl Default for ListApplicationKeysResponse {
    fn default() -> Self {
        Self::new()
    }
}
