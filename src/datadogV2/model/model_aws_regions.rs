// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// AWS Regions to collect data from
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AWSRegions {
    AWSRegionsIncludeAll(Box<crate::datadogV2::model::AWSRegionsIncludeAll>),
    AWSRegionsIncludeOnly(Box<crate::datadogV2::model::AWSRegionsIncludeOnly>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AWSRegions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::AWSRegionsIncludeAll>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(AWSRegions::AWSRegionsIncludeAll(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::AWSRegionsIncludeOnly>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(AWSRegions::AWSRegionsIncludeOnly(_v));
            }
        }

        return Ok(AWSRegions::UnparsedObject(crate::datadog::UnparsedObject {
            value,
        }));
    }
}
