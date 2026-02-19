// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// An included resource item in the change request response.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ChangeRequestIncludedItem {
    ChangeRequestIncludedUser(Box<crate::datadogV2::model::ChangeRequestIncludedUser>),
    ChangeRequestIncludedDecision(Box<crate::datadogV2::model::ChangeRequestIncludedDecision>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ChangeRequestIncludedItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ChangeRequestIncludedUser>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ChangeRequestIncludedItem::ChangeRequestIncludedUser(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ChangeRequestIncludedDecision>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ChangeRequestIncludedItem::ChangeRequestIncludedDecision(_v));
            }
        }

        return Ok(ChangeRequestIncludedItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
