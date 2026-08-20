// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A retention interval, either aligned to calendar boundaries or of a fixed length.
/// Cohort criteria use calendar intervals; return criteria use fixed intervals.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ProductAnalyticsRetentionTimeInterval {
    ProductAnalyticsRetentionCalendarTimeInterval(
        Box<crate::datadogV2::model::ProductAnalyticsRetentionCalendarTimeInterval>,
    ),
    ProductAnalyticsRetentionFixedTimeInterval(
        Box<crate::datadogV2::model::ProductAnalyticsRetentionFixedTimeInterval>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ProductAnalyticsRetentionTimeInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ProductAnalyticsRetentionCalendarTimeInterval>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ProductAnalyticsRetentionTimeInterval::ProductAnalyticsRetentionCalendarTimeInterval(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ProductAnalyticsRetentionFixedTimeInterval>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ProductAnalyticsRetentionTimeInterval::ProductAnalyticsRetentionFixedTimeInterval(_v));
            }
        }

        return Ok(ProductAnalyticsRetentionTimeInterval::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
