// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// An included resource related to a status page email access policy.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StatusPageEmailAccessPolicyIncluded {
    StatusPagesUser(Box<crate::datadogV2::model::StatusPagesUser>),
    StatusPageAsIncluded(Box<crate::datadogV2::model::StatusPageAsIncluded>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for StatusPageEmailAccessPolicyIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::StatusPagesUser>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(StatusPageEmailAccessPolicyIncluded::StatusPagesUser(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::StatusPageAsIncluded>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(StatusPageEmailAccessPolicyIncluded::StatusPageAsIncluded(
                    _v,
                ));
            }
        }

        return Ok(StatusPageEmailAccessPolicyIncluded::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
