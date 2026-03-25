// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The type of aggregation that can be performed on events-based queries.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum EventsAggregation {
    EventsAggregationValue(Box<crate::datadogV1::model::EventsAggregationValue>),
    EventsAggregationPercentile(String),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for EventsAggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::EventsAggregationValue>>(
            value.clone(),
        ) {
            match *_v {
                crate::datadogV1::model::EventsAggregationValue::UnparsedObject(_v) => {}
                _ => return Ok(EventsAggregation::EventsAggregationValue(_v)),
            }
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(EventsAggregation::EventsAggregationPercentile(_v));
        }

        return Ok(EventsAggregation::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
