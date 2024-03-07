// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TreeMapSizeBy {
    #[serde(rename = "pct_cpu")]
    PCT_CPU,
    #[serde(rename = "pct_mem")]
    PCT_MEM,
}
impl ToString for TreeMapSizeBy {
    fn to_string(&self) -> String {
        match self {
            Self::PCT_CPU => String::from("pct_cpu"),
            Self::PCT_MEM => String::from("pct_mem"),
        }
    }
}
