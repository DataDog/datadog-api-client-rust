// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// SetOnDemandConcurrencyCapParams is a struct for passing parameters to the method [`SetOnDemandConcurrencyCap`]
#[derive(Clone, Debug)]
pub struct SetOnDemandConcurrencyCapParams {
    /* . */
    pub body: OnDemandConcurrencyCapAttributes,
}




/// GetOnDemandConcurrencyCapError is a struct for typed errors of method [`GetOnDemandConcurrencyCap`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOnDemandConcurrencyCapError {
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SetOnDemandConcurrencyCapError is a struct for typed errors of method [`SetOnDemandConcurrencyCap`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetOnDemandConcurrencyCapError {
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}