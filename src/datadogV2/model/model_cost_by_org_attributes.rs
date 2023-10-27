// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CostByOrgAttributes {
    /// List of charges data reported for the requested month.
    #[serde(rename = "charges")]
    pub charges: Option<Vec<crate::datadogV2::model::ChargebackBreakdown>>,
    /// The month requested.
    #[serde(rename = "date")]
    pub date: Option<String>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// The total cost of products for the month.
    #[serde(rename = "total_cost")]
    pub total_cost: Option<f64>,
}

impl CostByOrgAttributes {
    /// Cost attributes data.
    pub fn new() -> CostByOrgAttributes {
        CostByOrgAttributes {
            charges: None,
            date: None,
            org_name: None,
            public_id: None,
            region: None,
            total_cost: None,
        }
    }
}
