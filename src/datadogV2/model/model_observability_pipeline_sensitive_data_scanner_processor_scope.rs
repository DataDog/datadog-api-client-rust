// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Determines which parts of the log the pattern-matching rule should be applied to.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineSensitiveDataScannerProcessorScope {
    ObservabilityPipelineSensitiveDataScannerProcessorScopeInclude(
        Box<
            crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScopeInclude,
        >,
    ),
    ObservabilityPipelineSensitiveDataScannerProcessorScopeExclude(
        Box<
            crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScopeExclude,
        >,
    ),
    ObservabilityPipelineSensitiveDataScannerProcessorScopeAll(
        Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScopeAll>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineSensitiveDataScannerProcessorScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScopeInclude>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineSensitiveDataScannerProcessorScope::ObservabilityPipelineSensitiveDataScannerProcessorScopeInclude(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScopeExclude>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineSensitiveDataScannerProcessorScope::ObservabilityPipelineSensitiveDataScannerProcessorScopeExclude(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<
                crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScopeAll,
            >,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineSensitiveDataScannerProcessorScope::ObservabilityPipelineSensitiveDataScannerProcessorScopeAll(_v));
            }
        }

        return Ok(
            ObservabilityPipelineSensitiveDataScannerProcessorScope::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
