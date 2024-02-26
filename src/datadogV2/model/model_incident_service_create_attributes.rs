// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident service's attributes for a create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentServiceCreateAttributes {
    /// Name of the incident service.
    #[serde(rename = "name")]
    pub name: String,
}

impl IncidentServiceCreateAttributes {
    pub fn new(name: String) -> IncidentServiceCreateAttributes {
        IncidentServiceCreateAttributes { name }
    }
}
