// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Object describing an assertion for a Network Path test.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SyntheticsNetworkAssertion {
    SyntheticsNetworkAssertionLatency(
        Box<crate::datadogV2::model::SyntheticsNetworkAssertionLatency>,
    ),
    SyntheticsNetworkAssertionMultiNetworkHop(
        Box<crate::datadogV2::model::SyntheticsNetworkAssertionMultiNetworkHop>,
    ),
    SyntheticsNetworkAssertionPacketLossPercentage(
        Box<crate::datadogV2::model::SyntheticsNetworkAssertionPacketLossPercentage>,
    ),
    SyntheticsNetworkAssertionJitter(
        Box<crate::datadogV2::model::SyntheticsNetworkAssertionJitter>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SyntheticsNetworkAssertion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SyntheticsNetworkAssertionLatency>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SyntheticsNetworkAssertion::SyntheticsNetworkAssertionLatency(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SyntheticsNetworkAssertionMultiNetworkHop>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    SyntheticsNetworkAssertion::SyntheticsNetworkAssertionMultiNetworkHop(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SyntheticsNetworkAssertionPacketLossPercentage>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    SyntheticsNetworkAssertion::SyntheticsNetworkAssertionPacketLossPercentage(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SyntheticsNetworkAssertionJitter>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SyntheticsNetworkAssertion::SyntheticsNetworkAssertionJitter(_v));
            }
        }

        return Ok(SyntheticsNetworkAssertion::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
