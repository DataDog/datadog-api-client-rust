// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident attachment's relationships.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentRelationships {
    /// Relationship to user.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: Option<Box<crate::datadogV2::model::RelationshipToUser>>,
}

impl IncidentAttachmentRelationships {
    pub fn new() -> IncidentAttachmentRelationships {
        IncidentAttachmentRelationships {
            last_modified_by_user: None,
        }
    }
}
impl Default for IncidentAttachmentRelationships {
    fn default() -> Self {
        Self::new()
    }
}
