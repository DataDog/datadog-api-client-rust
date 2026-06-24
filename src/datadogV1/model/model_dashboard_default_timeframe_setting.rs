// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The default timeframe applied when opening the dashboard. Set to `null` to clear the dashboard's default timeframe.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DashboardDefaultTimeframeSetting {
    DashboardLiveTimeframe(Box<crate::datadogV1::model::DashboardLiveTimeframe>),
    DashboardFixedTimeframe(Box<crate::datadogV1::model::DashboardFixedTimeframe>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DashboardDefaultTimeframeSetting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::DashboardLiveTimeframe>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(DashboardDefaultTimeframeSetting::DashboardLiveTimeframe(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::DashboardFixedTimeframe>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DashboardDefaultTimeframeSetting::DashboardFixedTimeframe(
                    _v,
                ));
            }
        }

        return Ok(DashboardDefaultTimeframeSetting::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
