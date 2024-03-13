// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Object to handle basic authentication when performing the test.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SyntheticsBasicAuth {
    SyntheticsBasicAuthWeb(Box<crate::datadogV1::model::SyntheticsBasicAuthWeb>),
    SyntheticsBasicAuthSigv4(Box<crate::datadogV1::model::SyntheticsBasicAuthSigv4>),
    SyntheticsBasicAuthNTLM(Box<crate::datadogV1::model::SyntheticsBasicAuthNTLM>),
    SyntheticsBasicAuthDigest(Box<crate::datadogV1::model::SyntheticsBasicAuthDigest>),
    SyntheticsBasicAuthOauthClient(Box<crate::datadogV1::model::SyntheticsBasicAuthOauthClient>),
    SyntheticsBasicAuthOauthROP(Box<crate::datadogV1::model::SyntheticsBasicAuthOauthROP>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SyntheticsBasicAuth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::SyntheticsBasicAuthWeb>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(SyntheticsBasicAuth::SyntheticsBasicAuthWeb(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SyntheticsBasicAuthSigv4>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SyntheticsBasicAuth::SyntheticsBasicAuthSigv4(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SyntheticsBasicAuthNTLM>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SyntheticsBasicAuth::SyntheticsBasicAuthNTLM(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SyntheticsBasicAuthDigest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SyntheticsBasicAuth::SyntheticsBasicAuthDigest(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SyntheticsBasicAuthOauthClient>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SyntheticsBasicAuth::SyntheticsBasicAuthOauthClient(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SyntheticsBasicAuthOauthROP>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SyntheticsBasicAuth::SyntheticsBasicAuthOauthROP(_v));
            }
        }

        return Ok(SyntheticsBasicAuth::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
