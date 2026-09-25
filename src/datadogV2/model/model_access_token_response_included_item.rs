// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// An object related to an access token.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AccessTokenResponseIncludedItem {
    LeakedKey(Box<crate::datadogV2::model::LeakedKey>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AccessTokenResponseIncludedItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::LeakedKey>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(AccessTokenResponseIncludedItem::LeakedKey(_v));
            }
        }

        return Ok(AccessTokenResponseIncludedItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
