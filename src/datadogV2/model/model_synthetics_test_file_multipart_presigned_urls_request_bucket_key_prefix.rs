// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsTestFileMultipartPresignedUrlsRequestBucketKeyPrefix {
    API_UPLOAD_FILE,
    BROWSER_UPLOAD_FILE_STEP,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SyntheticsTestFileMultipartPresignedUrlsRequestBucketKeyPrefix {
    fn to_string(&self) -> String {
        match self {
            Self::API_UPLOAD_FILE => String::from("api-upload-file"),
            Self::BROWSER_UPLOAD_FILE_STEP => String::from("browser-upload-file-step"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsTestFileMultipartPresignedUrlsRequestBucketKeyPrefix {
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

impl<'de> Deserialize<'de> for SyntheticsTestFileMultipartPresignedUrlsRequestBucketKeyPrefix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "api-upload-file" => Self::API_UPLOAD_FILE,
            "browser-upload-file-step" => Self::BROWSER_UPLOAD_FILE_STEP,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
