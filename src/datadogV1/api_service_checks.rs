// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// SubmitServiceCheckParams is a struct for passing parameters to the method [`SubmitServiceCheck`]
#[derive(Clone, Debug)]
pub struct SubmitServiceCheckParams {
    /* Service Check request body. */
    pub body: Vec<ServiceCheck>,
}




/// SubmitServiceCheckError is a struct for typed errors of method [`SubmitServiceCheck`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitServiceCheckError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status408(APIErrorResponse),
    Status413(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}