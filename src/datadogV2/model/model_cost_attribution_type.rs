// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CostAttributionType {
    #[serde(rename = "cost_by_tag")]
    COST_BY_TAG,
}

impl ToString for CostAttributionType {
    fn to_string(&self) -> String {
        match self {
            Self::COST_BY_TAG => String::from("cost_by_tag"),
        }
    }
}
