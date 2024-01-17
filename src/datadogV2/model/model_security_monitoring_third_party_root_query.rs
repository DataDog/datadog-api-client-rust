// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A query to be combined with the third party case query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringThirdPartyRootQuery {
    /// Fields to group by.
    #[serde(rename = "groupByFields")]
    pub group_by_fields: Option<Vec<String>>,
    /// Query to run on logs.
    #[serde(rename = "query")]
    pub query: Option<String>,
}

impl SecurityMonitoringThirdPartyRootQuery {
    pub fn new() -> SecurityMonitoringThirdPartyRootQuery {
        SecurityMonitoringThirdPartyRootQuery {
            group_by_fields: None,
            query: None,
        }
    }
}
