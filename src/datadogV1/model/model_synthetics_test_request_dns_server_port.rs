// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// DNS server port to use for DNS tests.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SyntheticsTestRequestDNSServerPort {
    SyntheticsTestRequestNumericalDNSServerPort(i64),
    SyntheticsTestRequestVariableDNSServerPort(String),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SyntheticsTestRequestDNSServerPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<i64>(value.clone()) {
            return Ok(
                SyntheticsTestRequestDNSServerPort::SyntheticsTestRequestNumericalDNSServerPort(_v),
            );
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(
                SyntheticsTestRequestDNSServerPort::SyntheticsTestRequestVariableDNSServerPort(_v),
            );
        }

        return Ok(SyntheticsTestRequestDNSServerPort::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
