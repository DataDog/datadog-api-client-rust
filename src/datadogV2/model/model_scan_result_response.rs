// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The raw scan result document produced by the SCA processor.
/// The contents reflect the vulnerabilities and metadata produced for the libraries
/// submitted in the original scan request.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ScanResultResponse {
    AnyValueObject(std::collections::BTreeMap<String, serde_json::Value>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ScanResultResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            std::collections::BTreeMap<String, serde_json::Value>,
        >(value.clone())
        {
            return Ok(ScanResultResponse::AnyValueObject(_v));
        }

        return Ok(ScanResultResponse::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
