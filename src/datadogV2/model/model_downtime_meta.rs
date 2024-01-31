// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Pagination metadata returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeMeta {
    /// Object containing the total filtered count.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::DowntimeMetaPage>,
}

impl DowntimeMeta {
    pub fn new() -> DowntimeMeta {
        DowntimeMeta { page: None }
    }

    pub fn with_page(&mut self, value: crate::datadogV2::model::DowntimeMetaPage) -> &mut Self {
        self.page = Some(value);
        self
    }
}
impl Default for DowntimeMeta {
    fn default() -> Self {
        Self::new()
    }
}
