// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing document metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MonthlyUsageAttributionMetadata {
    /// An array of available aggregates.
    #[serde(rename = "aggregates")]
    pub aggregates: Option<Vec<crate::datadogV1::model::UsageAttributionAggregatesBody>>,
    /// The metadata for the current pagination.
    #[serde(rename = "pagination")]
    pub pagination: Option<Box<crate::datadogV1::model::MonthlyUsageAttributionPagination>>,
}

impl MonthlyUsageAttributionMetadata {
    pub fn new() -> MonthlyUsageAttributionMetadata {
        MonthlyUsageAttributionMetadata {
            aggregates: None,
            pagination: None,
        }
    }
}