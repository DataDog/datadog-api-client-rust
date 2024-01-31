// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The form data submitted to upload IdP metadata
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdPMetadataFormData {
    /// The IdP metadata XML file
    #[serde(rename = "idp_file")]
    pub idp_file: Option<Vec<u8>>,
}

impl IdPMetadataFormData {
    pub fn new() -> IdPMetadataFormData {
        IdPMetadataFormData { idp_file: None }
    }

    pub fn with_idp_file(&mut self, value: Vec<u8>) -> &mut Self {
        self.idp_file = Some(value);
        self
    }
}
impl Default for IdPMetadataFormData {
    fn default() -> Self {
        Self::new()
    }
}
