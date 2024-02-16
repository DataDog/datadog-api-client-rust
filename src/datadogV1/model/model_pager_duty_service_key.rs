// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// PagerDuty service object key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagerDutyServiceKey {
    /// Your service key in PagerDuty.
    #[serde(rename = "service_key")]
    pub service_key: String,
}

impl PagerDutyServiceKey {
    pub fn new(service_key: String) -> PagerDutyServiceKey {
        PagerDutyServiceKey { service_key }
    }
}
