// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// AWS Authentication config.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AWSAuthConfig {
    AWSAuthConfigKeys(Box<crate::datadogV2::model::AWSAuthConfigKeys>),
    AWSAuthConfigRole(Box<crate::datadogV2::model::AWSAuthConfigRole>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AWSAuthConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::AWSAuthConfigKeys>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(AWSAuthConfig::AWSAuthConfigKeys(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::AWSAuthConfigRole>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(AWSAuthConfig::AWSAuthConfigRole(_v));
            }
        }

        return Ok(AWSAuthConfig::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
