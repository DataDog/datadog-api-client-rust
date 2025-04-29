// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityPipelineOcsfMappingLibrary {
    CLOUDTRAIL_ACCOUNT_CHANGE,
    GCP_CLOUD_AUDIT_CREATEBUCKET,
    GCP_CLOUD_AUDIT_CREATESINK,
    GCP_CLOUD_AUDIT_SETIAMPOLICY,
    GCP_CLOUD_AUDIT_UPDATESINK,
    GITHUB_AUDIT_LOG_API_ACTIVITY,
    GOOGLE_WORKSPACE_ADMIN_AUDIT_ADDPRIVILEGE,
    MICROSOFT_365_DEFENDER_INCIDENT,
    MICROSOFT_365_DEFENDER_USERLOGGEDIN,
    OKTA_SYSTEM_LOG_AUTHENTICATION,
    PALO_ALTO_NETWORKS_FIREWALL_TRAFFIC,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ObservabilityPipelineOcsfMappingLibrary {
    fn to_string(&self) -> String {
        match self {
            Self::CLOUDTRAIL_ACCOUNT_CHANGE => String::from("CloudTrail Account Change"),
            Self::GCP_CLOUD_AUDIT_CREATEBUCKET => String::from("GCP Cloud Audit CreateBucket"),
            Self::GCP_CLOUD_AUDIT_CREATESINK => String::from("GCP Cloud Audit CreateSink"),
            Self::GCP_CLOUD_AUDIT_SETIAMPOLICY => String::from("GCP Cloud Audit SetIamPolicy"),
            Self::GCP_CLOUD_AUDIT_UPDATESINK => String::from("GCP Cloud Audit UpdateSink"),
            Self::GITHUB_AUDIT_LOG_API_ACTIVITY => String::from("Github Audit Log API Activity"),
            Self::GOOGLE_WORKSPACE_ADMIN_AUDIT_ADDPRIVILEGE => {
                String::from("Google Workspace Admin Audit addPrivilege")
            }
            Self::MICROSOFT_365_DEFENDER_INCIDENT => {
                String::from("Microsoft 365 Defender Incident")
            }
            Self::MICROSOFT_365_DEFENDER_USERLOGGEDIN => {
                String::from("Microsoft 365 Defender UserLoggedIn")
            }
            Self::OKTA_SYSTEM_LOG_AUTHENTICATION => String::from("Okta System Log Authentication"),
            Self::PALO_ALTO_NETWORKS_FIREWALL_TRAFFIC => {
                String::from("Palo Alto Networks Firewall Traffic")
            }
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ObservabilityPipelineOcsfMappingLibrary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineOcsfMappingLibrary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "CloudTrail Account Change" => Self::CLOUDTRAIL_ACCOUNT_CHANGE,
            "GCP Cloud Audit CreateBucket" => Self::GCP_CLOUD_AUDIT_CREATEBUCKET,
            "GCP Cloud Audit CreateSink" => Self::GCP_CLOUD_AUDIT_CREATESINK,
            "GCP Cloud Audit SetIamPolicy" => Self::GCP_CLOUD_AUDIT_SETIAMPOLICY,
            "GCP Cloud Audit UpdateSink" => Self::GCP_CLOUD_AUDIT_UPDATESINK,
            "Github Audit Log API Activity" => Self::GITHUB_AUDIT_LOG_API_ACTIVITY,
            "Google Workspace Admin Audit addPrivilege" => {
                Self::GOOGLE_WORKSPACE_ADMIN_AUDIT_ADDPRIVILEGE
            }
            "Microsoft 365 Defender Incident" => Self::MICROSOFT_365_DEFENDER_INCIDENT,
            "Microsoft 365 Defender UserLoggedIn" => Self::MICROSOFT_365_DEFENDER_USERLOGGEDIN,
            "Okta System Log Authentication" => Self::OKTA_SYSTEM_LOG_AUTHENTICATION,
            "Palo Alto Networks Firewall Traffic" => Self::PALO_ALTO_NETWORKS_FIREWALL_TRAFFIC,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
