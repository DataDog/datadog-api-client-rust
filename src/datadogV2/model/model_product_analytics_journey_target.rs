// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A reference to a step, or a range of steps, in the journey.
/// Use a `node` target to name a single step, or a `path` target to name the range
/// between two steps.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ProductAnalyticsJourneyTarget {
    ProductAnalyticsJourneyNodeTarget(
        Box<crate::datadogV2::model::ProductAnalyticsJourneyNodeTarget>,
    ),
    ProductAnalyticsJourneyPathTarget(
        Box<crate::datadogV2::model::ProductAnalyticsJourneyPathTarget>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ProductAnalyticsJourneyTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ProductAnalyticsJourneyNodeTarget>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ProductAnalyticsJourneyTarget::ProductAnalyticsJourneyNodeTarget(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ProductAnalyticsJourneyPathTarget>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ProductAnalyticsJourneyTarget::ProductAnalyticsJourneyPathTarget(_v));
            }
        }

        return Ok(ProductAnalyticsJourneyTarget::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
