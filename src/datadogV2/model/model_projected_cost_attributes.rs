// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Projected Cost attributes data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectedCostAttributes {
    /// List of charges data reported for the requested month.
    #[serde(rename = "charges")]
    pub charges: Option<Vec<crate::datadogV2::model::ChargebackBreakdown>>,
    /// The month requested.
    #[serde(rename = "date")]
    pub date: Option<String>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The total projected cost of products for the month.
    #[serde(rename = "projected_total_cost")]
    pub projected_total_cost: Option<f64>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region")]
    pub region: Option<String>,
}

impl ProjectedCostAttributes {
    pub fn new() -> ProjectedCostAttributes {
        ProjectedCostAttributes {
            charges: None,
            date: None,
            org_name: None,
            projected_total_cost: None,
            public_id: None,
            region: None,
        }
    }

    pub fn charges(
        &mut self,
        value: Vec<crate::datadogV2::model::ChargebackBreakdown>,
    ) -> &mut Self {
        self.charges = Some(value);
        self
    }

    pub fn date(&mut self, value: String) -> &mut Self {
        self.date = Some(value);
        self
    }

    pub fn org_name(&mut self, value: String) -> &mut Self {
        self.org_name = Some(value);
        self
    }

    pub fn projected_total_cost(&mut self, value: f64) -> &mut Self {
        self.projected_total_cost = Some(value);
        self
    }

    pub fn public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    pub fn region(&mut self, value: String) -> &mut Self {
        self.region = Some(value);
        self
    }
}

impl Default for ProjectedCostAttributes {
    fn default() -> Self {
        Self::new()
    }
}
