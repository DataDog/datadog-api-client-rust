// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A list of current AWS services for which Datadog offers automatic log collection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSLogsServicesRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Array of services IDs set to enable automatic log collection. Discover the list of available services with the get list of AWS log ready services API endpoint.
    #[serde(rename = "services")]
    pub services: Vec<String>,
}

impl AWSLogsServicesRequest {
    pub fn new(account_id: String, services: Vec<String>) -> AWSLogsServicesRequest {
        AWSLogsServicesRequest {
            account_id,
            services,
        }
    }
}
