// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditLogsResponseLinks {
    /// Link for the next set of results. Note that the request can also be made using the
    /// POST endpoint.
    #[serde(rename = "next")]
    pub next: Option<String>,
}

impl AuditLogsResponseLinks {
    /// Links attributes.
    pub fn new() -> AuditLogsResponseLinks {
        AuditLogsResponseLinks { next: None }
    }
}
