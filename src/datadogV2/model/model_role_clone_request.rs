// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request to create a role by cloning an existing role.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleCloneRequest {
    /// Data for the clone role request.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::RoleClone,
}

impl RoleCloneRequest {
    pub fn new(data: crate::datadogV2::model::RoleClone) -> RoleCloneRequest {
        RoleCloneRequest { data }
    }
}
