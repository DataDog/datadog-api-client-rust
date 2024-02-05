// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A monitor configuration policy data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyEditData {
    /// Policy and policy type for a monitor configuration policy.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::MonitorConfigPolicyAttributeEditRequest,
    /// ID of this monitor configuration policy.
    #[serde(rename = "id")]
    pub id: String,
    /// Monitor configuration policy resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::MonitorConfigPolicyResourceType,
}

impl MonitorConfigPolicyEditData {
    pub fn new(
        attributes: crate::datadogV2::model::MonitorConfigPolicyAttributeEditRequest,
        id: String,
        type_: crate::datadogV2::model::MonitorConfigPolicyResourceType,
    ) -> MonitorConfigPolicyEditData {
        MonitorConfigPolicyEditData {
            attributes,
            id,
            type_,
        }
    }
}
