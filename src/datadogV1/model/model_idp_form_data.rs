// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the IdP configuration.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IdpFormData {
    /// The path to the XML metadata file you wish to upload.
    #[serde(rename = "idp_file")]
    pub idp_file: Vec<u8>,
}

impl IdpFormData {
    pub fn new(idp_file: Vec<u8>) -> IdpFormData {
        IdpFormData { idp_file }
    }
}
