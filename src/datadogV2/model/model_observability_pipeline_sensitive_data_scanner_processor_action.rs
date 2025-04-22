// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The definition of `ObservabilityPipelineSensitiveDataScannerProcessorAction` object.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineSensitiveDataScannerProcessorAction {
    ObservabilityPipelineSensitiveDataScannerProcessorActionRedact(Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionRedact>),
	ObservabilityPipelineSensitiveDataScannerProcessorActionHash(Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionHash>),
	ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact(Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineSensitiveDataScannerProcessorAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionRedact>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineSensitiveDataScannerProcessorAction::ObservabilityPipelineSensitiveDataScannerProcessorActionRedact(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionHash>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineSensitiveDataScannerProcessorAction::ObservabilityPipelineSensitiveDataScannerProcessorActionHash(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineSensitiveDataScannerProcessorAction::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact(_v));
            }
        }

        return Ok(
            ObservabilityPipelineSensitiveDataScannerProcessorAction::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
