// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship to incident responders.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToIncidentResponders {
    /// An array of incident responders.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::RelationshipToIncidentResponderData>,
}

impl RelationshipToIncidentResponders {
    pub fn new(
        data: Vec<crate::datadogV2::model::RelationshipToIncidentResponderData>,
    ) -> RelationshipToIncidentResponders {
        RelationshipToIncidentResponders { data }
    }
}