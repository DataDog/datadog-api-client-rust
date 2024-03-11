// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// An individual timeseries query to one of the basic Datadog data sources.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TimeseriesQuery {
    MetricsTimeseriesQuery(Box<crate::datadogV2::model::MetricsTimeseriesQuery>),
    EventsTimeseriesQuery(Box<crate::datadogV2::model::EventsTimeseriesQuery>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for TimeseriesQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::MetricsTimeseriesQuery>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(TimeseriesQuery::MetricsTimeseriesQuery(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::EventsTimeseriesQuery>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(TimeseriesQuery::EventsTimeseriesQuery(_v));
            }
        }

        return Ok(TimeseriesQuery::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
