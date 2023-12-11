// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The PagerDuty service that is available for integration with Datadog.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PagerDutyService {
    /// Your service key in PagerDuty.
    #[serde(rename = "service_key")]
    pub service_key: String,
    /// Your service name associated with a service key in PagerDuty.
    #[serde(rename = "service_name")]
    pub service_name: String,
}

impl PagerDutyService {
    pub fn new(service_key: String, service_name: String) -> PagerDutyService {
        PagerDutyService {
            service_key,
            service_name,
        }
    }
}
