// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A query definition discriminated by the `data_source` field.
/// Use `product_analytics` for standard event queries, or
/// `product_analytics_occurrence` for occurrence-filtered queries.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ProductAnalyticsBaseQuery {
    ProductAnalyticsEventQuery(Box<crate::datadogV2::model::ProductAnalyticsEventQuery>),
    ProductAnalyticsOccurrenceQuery(Box<crate::datadogV2::model::ProductAnalyticsOccurrenceQuery>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ProductAnalyticsBaseQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ProductAnalyticsEventQuery>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ProductAnalyticsBaseQuery::ProductAnalyticsEventQuery(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ProductAnalyticsOccurrenceQuery>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ProductAnalyticsBaseQuery::ProductAnalyticsOccurrenceQuery(
                    _v,
                ));
            }
        }

        return Ok(ProductAnalyticsBaseQuery::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
