// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing document metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyCostAttributionMeta {
    /// An array of available aggregates.
    #[serde(rename = "aggregates")]
    pub aggregates: Option<Vec<crate::datadogV2::model::CostAttributionAggregatesBody>>,
    /// The metadata for the current pagination.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV2::model::MonthlyCostAttributionPagination>,
}

impl MonthlyCostAttributionMeta {
    pub fn new() -> MonthlyCostAttributionMeta {
        MonthlyCostAttributionMeta {
            aggregates: None,
            pagination: None,
        }
    }

    pub fn with_aggregates(
        &mut self,
        value: Vec<crate::datadogV2::model::CostAttributionAggregatesBody>,
    ) -> &mut Self {
        self.aggregates = Some(value);
        self
    }

    pub fn with_pagination(
        &mut self,
        value: crate::datadogV2::model::MonthlyCostAttributionPagination,
    ) -> &mut Self {
        self.pagination = Some(value);
        self
    }
}
impl Default for MonthlyCostAttributionMeta {
    fn default() -> Self {
        Self::new()
    }
}
