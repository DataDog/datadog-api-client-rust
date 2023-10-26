// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditLogsEventAttributes {
    /// JSON object of attributes from Audit Logs events.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::datadogV2::model::None>>,
    /// Message of the event.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Name of the application or service generating Audit Logs events.
    /// This name is used to correlate Audit Logs to APM, so make sure you specify the same
    /// value when you use both products.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// Array of tags associated with your event.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Timestamp of your event.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl AuditLogsEventAttributes {
    /// JSON object containing all event attributes and their associated values.
    pub fn new() -> AuditLogsEventAttributes {
        AuditLogsEventAttributes {
            attributes: None,
            message: None,
            service: None,
            tags: None,
            timestamp: None,
        }
    }
}
