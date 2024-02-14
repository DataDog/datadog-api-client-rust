// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Additional information related to the application key response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKeyResponseMeta {
    /// Max allowed number of application keys per user.
    #[serde(rename = "max_allowed_per_user")]
    pub max_allowed_per_user: Option<i64>,
    /// Additional information related to the application key response.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::ApplicationKeyResponseMetaPage>,
}

impl ApplicationKeyResponseMeta {
    pub fn new() -> ApplicationKeyResponseMeta {
        ApplicationKeyResponseMeta {
            max_allowed_per_user: None,
            page: None,
        }
    }

    pub fn max_allowed_per_user(&mut self, value: i64) -> &mut Self {
        self.max_allowed_per_user = Some(value);
        self
    }

    pub fn page(
        &mut self,
        value: crate::datadogV2::model::ApplicationKeyResponseMetaPage,
    ) -> &mut Self {
        self.page = Some(value);
        self
    }
}

impl Default for ApplicationKeyResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}
