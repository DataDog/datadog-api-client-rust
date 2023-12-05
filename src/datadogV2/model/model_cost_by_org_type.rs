// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CostByOrgType {
    #[serde(rename = "cost_by_org")]
    COST_BY_ORG,
}

impl ToString for CostByOrgType {
    fn to_string(&self) -> String {
        match self {
            Self::COST_BY_ORG => String::from("cost_by_org"),
        }
    }
}

impl Default for CostByOrgType {
    fn default() -> CostByOrgType {
        Self::COST_BY_ORG
    }
}