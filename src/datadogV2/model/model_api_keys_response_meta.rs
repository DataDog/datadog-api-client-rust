// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Additional information related to api keys response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIKeysResponseMeta {
    /// Max allowed number of API keys.
    #[serde(rename = "max_allowed")]
    pub max_allowed: Option<i64>,
    /// Additional information related to the API keys response.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::APIKeysResponseMetaPage>>,
}

impl APIKeysResponseMeta {
    pub fn new() -> APIKeysResponseMeta {
        APIKeysResponseMeta {
            max_allowed: None,
            page: None,
        }
    }
}
impl Default for APIKeysResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}
