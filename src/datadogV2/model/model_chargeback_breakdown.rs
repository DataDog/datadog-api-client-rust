// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Charges breakdown.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargebackBreakdown {
    /// The type of charge for a particular product.
    #[serde(rename = "charge_type")]
    pub charge_type: Option<String>,
    /// The cost for a particular product and charge type during a given month.
    #[serde(rename = "cost")]
    pub cost: Option<f64>,
    /// The product for which cost is being reported.
    #[serde(rename = "product_name")]
    pub product_name: Option<String>,
}

impl ChargebackBreakdown {
    pub fn new() -> ChargebackBreakdown {
        ChargebackBreakdown {
            charge_type: None,
            cost: None,
            product_name: None,
        }
    }
}
