// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuditLogsEventType {
    #[serde(rename = "audit")]
	Audit,
}

impl ToString for AuditLogsEventType {
    fn to_string(&self) -> String {
        match self {
            Self::Audit => String::from("audit"),
        }
    }
}

impl Default for AuditLogsEventType {
    fn default() -> AuditLogsEventType {
        Self::Audit
    }
}
