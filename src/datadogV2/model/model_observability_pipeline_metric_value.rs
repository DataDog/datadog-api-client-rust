// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Specifies how the value of the generated metric is computed.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineMetricValue {
    ObservabilityPipelineGeneratedMetricIncrementByOne(
        Box<crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByOne>,
    ),
    ObservabilityPipelineGeneratedMetricIncrementByField(
        Box<crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByField>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineMetricValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByOne>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineMetricValue::ObservabilityPipelineGeneratedMetricIncrementByOne(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByField>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineMetricValue::ObservabilityPipelineGeneratedMetricIncrementByField(_v));
            }
        }

        return Ok(ObservabilityPipelineMetricValue::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
