// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityPipelineGoogleCloudStorageDestinationAcl {
    PRIVATE,
    PROJECTNOT_PRIVATE,
    PUBLICNOT_READ,
    AUTHENTICATEDNOT_READ,
    BUCKETNOT_OWNERNOT_READ,
    BUCKETNOT_OWNERNOT_FULLNOT_CONTROL,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ObservabilityPipelineGoogleCloudStorageDestinationAcl {
    fn to_string(&self) -> String {
        match self {
            Self::PRIVATE => String::from("private"),
            Self::PROJECTNOT_PRIVATE => String::from("project-private"),
            Self::PUBLICNOT_READ => String::from("public-read"),
            Self::AUTHENTICATEDNOT_READ => String::from("authenticated-read"),
            Self::BUCKETNOT_OWNERNOT_READ => String::from("bucket-owner-read"),
            Self::BUCKETNOT_OWNERNOT_FULLNOT_CONTROL => String::from("bucket-owner-full-control"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ObservabilityPipelineGoogleCloudStorageDestinationAcl {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineGoogleCloudStorageDestinationAcl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "private" => Self::PRIVATE,
            "project-private" => Self::PROJECTNOT_PRIVATE,
            "public-read" => Self::PUBLICNOT_READ,
            "authenticated-read" => Self::AUTHENTICATEDNOT_READ,
            "bucket-owner-read" => Self::BUCKETNOT_OWNERNOT_READ,
            "bucket-owner-full-control" => Self::BUCKETNOT_OWNERNOT_FULLNOT_CONTROL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
