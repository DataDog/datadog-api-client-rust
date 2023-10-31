// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A metric-based SLO. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator
/// to be used because this will sum up all request counts instead of averaging them, or taking the max or
/// min of all of those requests.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServiceLevelObjectiveQuery {
    /// A Datadog metric query for total (valid) events.
    #[serde(rename = "denominator")]
    pub denominator: String,
    /// A Datadog metric query for good events.
    #[serde(rename = "numerator")]
    pub numerator: String,
}

impl ServiceLevelObjectiveQuery {
    pub fn new(denominator: String, numerator: String) -> ServiceLevelObjectiveQuery {
        ServiceLevelObjectiveQuery { denominator, numerator }
    }
}
