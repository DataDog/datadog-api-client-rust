// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// PagerDuty service object name.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagerDutyServiceName {
    /// Your service name associated service key in PagerDuty.
    #[serde(rename = "service_name")]
    pub service_name: String,
}

impl PagerDutyServiceName {
    pub fn new(service_name: String) -> PagerDutyServiceName {
        PagerDutyServiceName { service_name }
    }
}
