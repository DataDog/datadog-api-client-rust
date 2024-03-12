// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// On-demand concurrency cap attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnDemandConcurrencyCapAttributes {
    /// Value of the on-demand concurrency cap.
    #[serde(rename = "on_demand_concurrency_cap")]
    pub on_demand_concurrency_cap: Option<f64>,
}

impl OnDemandConcurrencyCapAttributes {
    pub fn new() -> OnDemandConcurrencyCapAttributes {
        OnDemandConcurrencyCapAttributes {
            on_demand_concurrency_cap: None,
        }
    }

    pub fn on_demand_concurrency_cap(mut self, value: f64) -> Self {
        self.on_demand_concurrency_cap = Some(value);
        self
    }
}

impl Default for OnDemandConcurrencyCapAttributes {
    fn default() -> Self {
        Self::new()
    }
}
