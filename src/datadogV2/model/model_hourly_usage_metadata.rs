// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsageMetadata {
    /// The metadata for the current pagination.
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::datadogV2::model::HourlyUsagePagination>>,
}

impl HourlyUsageMetadata {
    /// The object containing document metadata.
    pub fn new() -> HourlyUsageMetadata {
        HourlyUsageMetadata { pagination: None }
    }
}
