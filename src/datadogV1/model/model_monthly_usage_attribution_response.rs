// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the monthly Usage Summary by tag(s).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyUsageAttributionResponse {
    /// The object containing document metadata.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::MonthlyUsageAttributionMetadata>,
    /// Get usage summary by tag(s).
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::MonthlyUsageAttributionBody>>,
}

impl MonthlyUsageAttributionResponse {
    pub fn new() -> MonthlyUsageAttributionResponse {
        MonthlyUsageAttributionResponse {
            metadata: None,
            usage: None,
        }
    }

    pub fn metadata(
        mut self,
        value: crate::datadogV1::model::MonthlyUsageAttributionMetadata,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn usage(
        mut self,
        value: Vec<crate::datadogV1::model::MonthlyUsageAttributionBody>,
    ) -> Self {
        self.usage = Some(value);
        self
    }
}

impl Default for MonthlyUsageAttributionResponse {
    fn default() -> Self {
        Self::new()
    }
}
