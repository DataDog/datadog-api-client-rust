// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnDemandConcurrencyCapAttributes {
    /// Value of the on-demand concurrency cap.
    #[serde(rename = "on_demand_concurrency_cap", skip_serializing_if = "Option::is_none")]
    pub on_demand_concurrency_cap: Option<f64>,
}

impl OnDemandConcurrencyCapAttributes {
    /// On-demand concurrency cap attributes.
    pub fn new() -> OnDemandConcurrencyCapAttributes {
        OnDemandConcurrencyCapAttributes {
            on_demand_concurrency_cap: None,
        }
    }
}
