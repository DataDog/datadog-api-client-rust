// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The definition of the `CloudflareCredentialsUpdate` object.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CloudflareCredentialsUpdate {
    CloudflareAPITokenUpdate(Box<crate::datadogV2::model::CloudflareAPITokenUpdate>),
    CloudflareGlobalAPITokenUpdate(Box<crate::datadogV2::model::CloudflareGlobalAPITokenUpdate>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CloudflareCredentialsUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CloudflareAPITokenUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CloudflareCredentialsUpdate::CloudflareAPITokenUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CloudflareGlobalAPITokenUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CloudflareCredentialsUpdate::CloudflareGlobalAPITokenUpdate(
                    _v,
                ));
            }
        }

        return Ok(CloudflareCredentialsUpdate::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
