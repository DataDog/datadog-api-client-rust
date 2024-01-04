// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Used to perform a histogram computation (only for measure facets).
/// Note: At most 100 buckets are allowed, the number of buckets is (max - min)/interval.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansGroupByHistogram {
    /// The bin size of the histogram buckets.
    #[serde(rename = "interval")]
    pub interval: f64,
    /// The maximum value for the measure used in the histogram
    /// (values greater than this one are filtered out).
    #[serde(rename = "max")]
    pub max: f64,
    /// The minimum value for the measure used in the histogram
    /// (values smaller than this one are filtered out).
    #[serde(rename = "min")]
    pub min: f64,
}

impl SpansGroupByHistogram {
    pub fn new(interval: f64, max: f64, min: f64) -> SpansGroupByHistogram {
        SpansGroupByHistogram { interval, max, min }
    }
}
