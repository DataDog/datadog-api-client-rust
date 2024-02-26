// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the monthly cost attribution by tag(s).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyCostAttributionResponse {
    /// Response containing cost attribution.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::MonthlyCostAttributionBody>>,
    /// The object containing document metadata.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::MonthlyCostAttributionMeta>,
}

impl MonthlyCostAttributionResponse {
    pub fn new() -> MonthlyCostAttributionResponse {
        MonthlyCostAttributionResponse {
            data: None,
            meta: None,
        }
    }

    pub fn data(
        &mut self,
        value: Vec<crate::datadogV2::model::MonthlyCostAttributionBody>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn meta(
        &mut self,
        value: crate::datadogV2::model::MonthlyCostAttributionMeta,
    ) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for MonthlyCostAttributionResponse {
    fn default() -> Self {
        Self::new()
    }
}
