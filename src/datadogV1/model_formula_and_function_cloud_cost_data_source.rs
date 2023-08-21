// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormulaAndFunctionCloudCostDataSource {
    #[serde(rename = "cloud_cost")]
	CLOUD_COST,
}

impl ToString for FormulaAndFunctionCloudCostDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::CLOUD_COST => String::from("cloud_cost"),
        }
    }
}

impl Default for FormulaAndFunctionCloudCostDataSource {
    fn default() -> FormulaAndFunctionCloudCostDataSource {
        Self::CLOUD_COST
    }
}
