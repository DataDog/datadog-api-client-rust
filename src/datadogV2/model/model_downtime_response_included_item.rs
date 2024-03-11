// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// An object related to a downtime.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DowntimeResponseIncludedItem {
    User(Box<crate::datadogV2::model::User>),
    DowntimeMonitorIncludedItem(Box<crate::datadogV2::model::DowntimeMonitorIncludedItem>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for DowntimeResponseIncludedItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::User>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(DowntimeResponseIncludedItem::User(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DowntimeMonitorIncludedItem>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DowntimeResponseIncludedItem::DowntimeMonitorIncludedItem(
                    _v,
                ));
            }
        }

        return Ok(DowntimeResponseIncludedItem::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
