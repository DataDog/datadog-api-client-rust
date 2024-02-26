// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DORADeploymentRequestData {
    /// Attributes to create a DORA deployment event.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::DORADeploymentRequestAttributes,
}

impl DORADeploymentRequestData {
    pub fn new(
        attributes: crate::datadogV2::model::DORADeploymentRequestAttributes,
    ) -> DORADeploymentRequestData {
        DORADeploymentRequestData { attributes }
    }
}
