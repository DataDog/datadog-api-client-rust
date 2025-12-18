// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Represents additional included resources for a on-call notification rules
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum OnCallNotificationRulesIncluded {
    NotificationChannelData(Box<crate::datadogV2::model::NotificationChannelData>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for OnCallNotificationRulesIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::NotificationChannelData>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(OnCallNotificationRulesIncluded::NotificationChannelData(_v));
            }
        }

        return Ok(OnCallNotificationRulesIncluded::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
