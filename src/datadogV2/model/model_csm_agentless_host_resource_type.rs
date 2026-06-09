// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CsmAgentlessHostResourceType {
    AWS_EC2_INSTANCE,
    AZURE_VIRTUAL_MACHINE_INSTANCE,
    GCP_COMPUTE_INSTANCE,
    OCI_INSTANCE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CsmAgentlessHostResourceType {
    fn to_string(&self) -> String {
        match self {
            Self::AWS_EC2_INSTANCE => String::from("aws_ec2_instance"),
            Self::AZURE_VIRTUAL_MACHINE_INSTANCE => String::from("azure_virtual_machine_instance"),
            Self::GCP_COMPUTE_INSTANCE => String::from("gcp_compute_instance"),
            Self::OCI_INSTANCE => String::from("oci_instance"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CsmAgentlessHostResourceType {
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

impl<'de> Deserialize<'de> for CsmAgentlessHostResourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "aws_ec2_instance" => Self::AWS_EC2_INSTANCE,
            "azure_virtual_machine_instance" => Self::AZURE_VIRTUAL_MACHINE_INSTANCE,
            "gcp_compute_instance" => Self::GCP_COMPUTE_INSTANCE,
            "oci_instance" => Self::OCI_INSTANCE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
