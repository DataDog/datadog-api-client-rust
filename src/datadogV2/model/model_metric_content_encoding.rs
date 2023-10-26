// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetricContentEncoding {
    #[serde(rename = "deflate")]
    DEFLATE,
    #[serde(rename = "zstd1")]
    ZSTD1,
    #[serde(rename = "gzip")]
    GZIP,
}

impl ToString for MetricContentEncoding {
    fn to_string(&self) -> String {
        match self {
            Self::DEFLATE => String::from("deflate"),
            Self::ZSTD1 => String::from("zstd1"),
            Self::GZIP => String::from("gzip"),
        }
    }
}

impl Default for MetricContentEncoding {
    fn default() -> MetricContentEncoding {
        Self::DEFLATE
    }
}
