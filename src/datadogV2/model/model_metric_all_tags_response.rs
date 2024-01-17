// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object that includes a single metric's indexed tags.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAllTagsResponse {
    /// Object for a single metric's indexed tags.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::MetricAllTags>>,
}

impl MetricAllTagsResponse {
    pub fn new() -> MetricAllTagsResponse {
        MetricAllTagsResponse { data: None }
    }
}
