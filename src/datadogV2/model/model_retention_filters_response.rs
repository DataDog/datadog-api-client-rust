// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An ordered list of retention filters.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RetentionFiltersResponse {
    /// A list of retention filters objects.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::RetentionFilterAll>,
}

impl RetentionFiltersResponse {
    pub fn new(data: Vec<crate::datadogV2::model::RetentionFilterAll>) -> RetentionFiltersResponse {
        RetentionFiltersResponse { data }
    }
}