// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CloudCostActivityType {
    #[serde(rename = "cloud_cost_activity")]
    CLOUD_COST_ACTIVITY,
}

impl ToString for CloudCostActivityType {
    fn to_string(&self) -> String {
        match self {
            Self::CLOUD_COST_ACTIVITY => String::from("cloud_cost_activity"),
        }
    }
}
