// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Attributes of a rule of a scanning group of a target. They depend on the type of rule.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SensitiveDataScannerTargetRuleAttributes {
    SensitiveDataScannerCustomRegexRuleAttributes(
        Box<crate::datadogV2::model::SensitiveDataScannerCustomRegexRuleAttributes>,
    ),
    SensitiveDataScannerStandardRegexRuleAttributes(
        Box<crate::datadogV2::model::SensitiveDataScannerStandardRegexRuleAttributes>,
    ),
    SensitiveDataScannerMachineLearningRuleAttributes(
        Box<crate::datadogV2::model::SensitiveDataScannerMachineLearningRuleAttributes>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SensitiveDataScannerTargetRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SensitiveDataScannerCustomRegexRuleAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SensitiveDataScannerTargetRuleAttributes::SensitiveDataScannerCustomRegexRuleAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SensitiveDataScannerStandardRegexRuleAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SensitiveDataScannerTargetRuleAttributes::SensitiveDataScannerStandardRegexRuleAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SensitiveDataScannerMachineLearningRuleAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SensitiveDataScannerTargetRuleAttributes::SensitiveDataScannerMachineLearningRuleAttributes(_v));
            }
        }

        return Ok(SensitiveDataScannerTargetRuleAttributes::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
