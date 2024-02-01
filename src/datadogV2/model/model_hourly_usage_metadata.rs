// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing document metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsageMetadata {
    /// The metadata for the current pagination.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV2::model::HourlyUsagePagination>,
}

impl HourlyUsageMetadata {
    pub fn new() -> HourlyUsageMetadata {
        HourlyUsageMetadata { pagination: None }
    }

    pub fn pagination(
        &mut self,
        value: crate::datadogV2::model::HourlyUsagePagination,
    ) -> &mut Self {
        self.pagination = Some(value);
        self
    }
}

impl Default for HourlyUsageMetadata {
    fn default() -> Self {
        Self::new()
    }
}
