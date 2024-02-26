// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The distribution points payload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionPointsPayload {
    /// A list of distribution points series to submit to Datadog.
    #[serde(rename = "series")]
    pub series: Vec<crate::datadogV1::model::DistributionPointsSeries>,
}

impl DistributionPointsPayload {
    pub fn new(
        series: Vec<crate::datadogV1::model::DistributionPointsSeries>,
    ) -> DistributionPointsPayload {
        DistributionPointsPayload { series }
    }
}
