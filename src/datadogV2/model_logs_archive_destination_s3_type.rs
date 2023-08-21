// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsArchiveDestinationS3Type {
    #[serde(rename = "s3")]
	S3,
}

impl ToString for LogsArchiveDestinationS3Type {
    fn to_string(&self) -> String {
        match self {
            Self::S3 => String::from("s3"),
        }
    }
}

impl Default for LogsArchiveDestinationS3Type {
    fn default() -> LogsArchiveDestinationS3Type {
        Self::S3
    }
}
