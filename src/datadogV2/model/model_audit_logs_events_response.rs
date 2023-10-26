// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditLogsEventsResponse {
    /// Array of events matching the request.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::datadogV2::model::AuditLogsEvent>>,
    /// Links attributes.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::datadogV2::model::AuditLogsResponseLinks>>,
    /// The metadata associated with a request.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::datadogV2::model::AuditLogsResponseMetadata>>,
}

impl AuditLogsEventsResponse {
    /// Response object with all events matching the request and pagination information.
    pub fn new() -> AuditLogsEventsResponse {
        AuditLogsEventsResponse {
            data: None,
            links: None,
            meta: None,
        }
    }
}
