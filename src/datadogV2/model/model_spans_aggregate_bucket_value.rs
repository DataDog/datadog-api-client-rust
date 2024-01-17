// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// A bucket value, can be either a timeseries or a single value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpansAggregateBucketValue {
    SpansAggregateBucketValueSingleString(Box<String>),
    SpansAggregateBucketValueSingleNumber(Box<f64>),
    SpansAggregateBucketValueTimeseries(
        Box<crate::datadogV2::model::SpansAggregateBucketValueTimeseries>,
    ),
}
