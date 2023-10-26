// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnDemandConcurrencyCapType {
    #[serde(rename = "on_demand_concurrency_cap")]
    ON_DEMAND_CONCURRENCY_CAP,
}

impl ToString for OnDemandConcurrencyCapType {
    fn to_string(&self) -> String {
        match self {
            Self::ON_DEMAND_CONCURRENCY_CAP => String::from("on_demand_concurrency_cap"),
        }
    }
}

impl Default for OnDemandConcurrencyCapType {
    fn default() -> OnDemandConcurrencyCapType {
        Self::ON_DEMAND_CONCURRENCY_CAP
    }
}
