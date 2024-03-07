// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FormulaAndFunctionApmDependencyStatsDataSource {
    #[serde(rename = "apm_dependency_stats")]
    APM_DEPENDENCY_STATS,
}
impl ToString for FormulaAndFunctionApmDependencyStatsDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::APM_DEPENDENCY_STATS => String::from("apm_dependency_stats"),
        }
    }
}
