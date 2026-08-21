// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Selects a cohort, either by index or by the aggregation that rolls all cohorts together.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ProductAnalyticsRetentionCohortTarget {
    ProductAnalyticsRetentionIndexTarget(
        Box<crate::datadogV2::model::ProductAnalyticsRetentionIndexTarget>,
    ),
    ProductAnalyticsRetentionAggregationTarget(
        Box<crate::datadogV2::model::ProductAnalyticsRetentionAggregationTarget>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ProductAnalyticsRetentionCohortTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ProductAnalyticsRetentionIndexTarget>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ProductAnalyticsRetentionCohortTarget::ProductAnalyticsRetentionIndexTarget(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ProductAnalyticsRetentionAggregationTarget>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ProductAnalyticsRetentionCohortTarget::ProductAnalyticsRetentionAggregationTarget(_v));
            }
        }

        return Ok(ProductAnalyticsRetentionCohortTarget::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
