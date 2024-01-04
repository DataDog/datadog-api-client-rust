// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the monthly cost attribution by tag(s).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyCostAttributionResponse {
    /// Response containing cost attribution.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::MonthlyCostAttributionBody>>,
    /// The object containing document metadata.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::MonthlyCostAttributionMeta>>,
}

impl MonthlyCostAttributionResponse {
    pub fn new() -> MonthlyCostAttributionResponse {
        MonthlyCostAttributionResponse {
            data: None,
            meta: None,
        }
    }
}
