// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Specifies where the worker extracts the token from in the incoming HTTP request.
/// This can be either a built-in location (`path` or `address`) or an HTTP header object.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineHttpServerSourceValidTokenPathToToken {
    ObservabilityPipelineHttpServerSourceValidTokenPathToTokenLocation(Box<crate::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToTokenLocation>),
	ObservabilityPipelineHttpServerSourceValidTokenPathToTokenHeader(Box<crate::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToTokenHeader>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineHttpServerSourceValidTokenPathToToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToTokenLocation>>(value.clone()) {
			match *_v {
				crate::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToTokenLocation::UnparsedObject(_v) => {},
				_ => {
					return Ok(ObservabilityPipelineHttpServerSourceValidTokenPathToToken::ObservabilityPipelineHttpServerSourceValidTokenPathToTokenLocation(_v))
				}
			}
		}
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToTokenHeader>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineHttpServerSourceValidTokenPathToToken::ObservabilityPipelineHttpServerSourceValidTokenPathToTokenHeader(_v));
            }
        }

        return Ok(
            ObservabilityPipelineHttpServerSourceValidTokenPathToToken::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
