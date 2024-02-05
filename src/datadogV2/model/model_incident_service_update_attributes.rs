// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident service's attributes for an update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentServiceUpdateAttributes {
    /// Name of the incident service.
    #[serde(rename = "name")]
    pub name: String,
}

impl IncidentServiceUpdateAttributes {
    pub fn new(name: String) -> IncidentServiceUpdateAttributes {
        IncidentServiceUpdateAttributes { name }
    }
}
