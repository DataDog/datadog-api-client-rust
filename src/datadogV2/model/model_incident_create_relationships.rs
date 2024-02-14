// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The relationships the incident will have with other resources once created.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentCreateRelationships {
    /// Relationship to user.
    #[serde(rename = "commander_user")]
    pub commander_user: Option<crate::datadogV2::model::NullableRelationshipToUser>,
}

impl IncidentCreateRelationships {
    pub fn new(
        commander_user: Option<crate::datadogV2::model::NullableRelationshipToUser>,
    ) -> IncidentCreateRelationships {
        IncidentCreateRelationships { commander_user }
    }
}
