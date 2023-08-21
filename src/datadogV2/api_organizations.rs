// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// UploadIdPMetadataParams is a struct for passing parameters to the method [`UploadIdPMetadata`]
#[derive(Clone, Debug)]
pub struct UploadIdPMetadataParams {
    /* The IdP metadata XML file */
    pub idp_file: *os.File,
}




/// UploadIdPMetadataError is a struct for typed errors of method [`UploadIdPMetadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadIdPMetadataError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}