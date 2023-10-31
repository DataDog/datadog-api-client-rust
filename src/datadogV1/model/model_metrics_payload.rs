// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metrics' payload.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MetricsPayload {
    /// A list of time series to submit to Datadog.
    #[serde(rename = "series")]
    pub series: Vec<crate::datadogV1::model::Series>,
}

impl MetricsPayload {
    pub fn new(series: Vec<crate::datadogV1::model::Series>) -> MetricsPayload {
        MetricsPayload { series }
    }
}
