// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Monitor identifier for the downtime.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DowntimeMonitorIdentifier {
    DowntimeMonitorIdentifierId(Box<crate::datadogV2::model::DowntimeMonitorIdentifierId>),
    DowntimeMonitorIdentifierTags(Box<crate::datadogV2::model::DowntimeMonitorIdentifierTags>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for DowntimeMonitorIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DowntimeMonitorIdentifierId>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DowntimeMonitorIdentifier::DowntimeMonitorIdentifierId(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DowntimeMonitorIdentifierTags>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DowntimeMonitorIdentifier::DowntimeMonitorIdentifierTags(_v));
            }
        }

        return Ok(DowntimeMonitorIdentifier::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
