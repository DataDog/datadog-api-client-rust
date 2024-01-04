// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A relationship reference for multiple integration metadata objects.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToIncidentIntegrationMetadatas {
    /// Integration metadata relationship array
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::RelationshipToIncidentIntegrationMetadataData>,
}

impl RelationshipToIncidentIntegrationMetadatas {
    pub fn new(
        data: Vec<crate::datadogV2::model::RelationshipToIncidentIntegrationMetadataData>,
    ) -> RelationshipToIncidentIntegrationMetadatas {
        RelationshipToIncidentIntegrationMetadatas { data }
    }
}