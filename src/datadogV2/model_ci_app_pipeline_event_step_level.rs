// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CIAppPipelineEventStepLevel {
    #[serde(rename = "step")]
	STEP,
}

impl ToString for CIAppPipelineEventStepLevel {
    fn to_string(&self) -> String {
        match self {
            Self::STEP => String::from("step"),
        }
    }
}

impl Default for CIAppPipelineEventStepLevel {
    fn default() -> CIAppPipelineEventStepLevel {
        Self::STEP
    }
}
