// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Specifies which principals are associated with a relation.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RestrictionPolicyBinding {
    /// An array of principals. A principal is a subject or group of subjects.
    /// Each principal is formatted as `type:id`. Supported types: `role`, `team`, `user`, and `org`.
    /// The org ID can be obtained through the api/v2/current_user API.
    /// The user principal type accepts service account IDs.
    #[serde(rename = "principals")]
    pub principals: Vec<String>,
    /// The role/level of access.
    #[serde(rename = "relation")]
    pub relation: String,
}

impl RestrictionPolicyBinding {
    pub fn new(principals: Vec<String>, relation: String) -> RestrictionPolicyBinding {
        RestrictionPolicyBinding {
            principals,
            relation,
        }
    }
}
