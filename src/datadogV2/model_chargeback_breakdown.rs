// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargebackBreakdown {
    /// The type of charge for a particular product.
    #[serde(rename = "charge_type", skip_serializing_if = "Option::is_none")]
    pub charge_type: String,
    /// The cost for a particular product and charge type during a given month.
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: f64,
    /// The product for which cost is being reported.
    #[serde(rename = "product_name", skip_serializing_if = "Option::is_none")]
    pub product_name: String,
}

