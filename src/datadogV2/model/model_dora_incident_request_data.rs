// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DORAIncidentRequestData {
    /// Attributes to create a DORA incident event.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::DORAIncidentRequestAttributes>,
}

impl DORAIncidentRequestData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::DORAIncidentRequestAttributes>,
    ) -> DORAIncidentRequestData {
        DORAIncidentRequestData { attributes }
    }
}
