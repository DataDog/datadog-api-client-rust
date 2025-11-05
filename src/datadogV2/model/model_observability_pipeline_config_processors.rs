// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A list of processors that transform or enrich log data, or a list of grouped processor configurations.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineConfigProcessors {
    ObservabilityPipelineConfigProcessorsList(
        Vec<crate::datadogV2::model::ObservabilityPipelineConfigProcessorItem>,
    ),
    ObservabilityPipelineConfigProcessorGroups(
        Vec<crate::datadogV2::model::ObservabilityPipelineConfigProcessorGroup>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineConfigProcessors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Vec<crate::datadogV2::model::ObservabilityPipelineConfigProcessorItem>,
        >(value.clone())
        {
            return Ok(
                ObservabilityPipelineConfigProcessors::ObservabilityPipelineConfigProcessorsList(
                    _v,
                ),
            );
        }
        if let Ok(_v) = serde_json::from_value::<
            Vec<crate::datadogV2::model::ObservabilityPipelineConfigProcessorGroup>,
        >(value.clone())
        {
            return Ok(
                ObservabilityPipelineConfigProcessors::ObservabilityPipelineConfigProcessorGroups(
                    _v,
                ),
            );
        }

        return Ok(ObservabilityPipelineConfigProcessors::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
