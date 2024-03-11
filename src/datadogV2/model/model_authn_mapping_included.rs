// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Included data in the AuthN Mapping response.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AuthNMappingIncluded {
    SAMLAssertionAttribute(Box<crate::datadogV2::model::SAMLAssertionAttribute>),
    Role(Box<crate::datadogV2::model::Role>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for AuthNMappingIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SAMLAssertionAttribute>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(AuthNMappingIncluded::SAMLAssertionAttribute(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::Role>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(AuthNMappingIncluded::Role(_v));
            }
        }

        return Ok(AuthNMappingIncluded::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
