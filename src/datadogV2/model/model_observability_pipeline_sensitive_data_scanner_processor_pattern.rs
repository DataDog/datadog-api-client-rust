// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Pattern detection configuration for identifying sensitive data using either a custom regex or a library reference.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineSensitiveDataScannerProcessorPattern {
    ObservabilityPipelineSensitiveDataScannerProcessorCustomPattern(Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorCustomPattern>),
	ObservabilityPipelineSensitiveDataScannerProcessorLibraryPattern(Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorLibraryPattern>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineSensitiveDataScannerProcessorPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorCustomPattern>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineSensitiveDataScannerProcessorPattern::ObservabilityPipelineSensitiveDataScannerProcessorCustomPattern(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorLibraryPattern>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineSensitiveDataScannerProcessorPattern::ObservabilityPipelineSensitiveDataScannerProcessorLibraryPattern(_v));
            }
        }

        return Ok(
            ObservabilityPipelineSensitiveDataScannerProcessorPattern::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
