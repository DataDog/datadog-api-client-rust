// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IdPMetadataFormData {
    /// The IdP metadata XML file
    #[serde(rename = "idp_file", skip_serializing_if = "Option::is_none")]
    pub idp_file: Option<Vec<u8>>,
}

impl IdPMetadataFormData {
    /// The form data submitted to upload IdP metadata
    pub fn new() -> IdPMetadataFormData {
        IdPMetadataFormData { idp_file: None }
    }
}
