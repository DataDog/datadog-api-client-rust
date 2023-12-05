// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the Usage Summary by tag(s).
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageAttributionResponse {
    /// The object containing document metadata.
    #[serde(rename = "metadata")]
    pub metadata: Option<Box<crate::datadogV1::model::UsageAttributionMetadata>>,
    /// Get usage summary by tag(s).
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageAttributionBody>>,
}

impl UsageAttributionResponse {
    pub fn new() -> UsageAttributionResponse {
        UsageAttributionResponse {
            metadata: None,
            usage: None,
        }
    }
}