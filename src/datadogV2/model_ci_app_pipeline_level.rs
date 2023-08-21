// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CIAppPipelineLevel {
    #[serde(rename = "pipeline")]
	PIPELINE,
    #[serde(rename = "stage")]
	STAGE,
    #[serde(rename = "job")]
	JOB,
    #[serde(rename = "step")]
	STEP,
    #[serde(rename = "custom")]
	CUSTOM,
}

impl ToString for CIAppPipelineLevel {
    fn to_string(&self) -> String {
        match self {
            Self::PIPELINE => String::from("pipeline"),
            Self::STAGE => String::from("stage"),
            Self::JOB => String::from("job"),
            Self::STEP => String::from("step"),
            Self::CUSTOM => String::from("custom"),
        }
    }
}

impl Default for CIAppPipelineLevel {
    fn default() -> CIAppPipelineLevel {
        Self::PIPELINE
    }
}
