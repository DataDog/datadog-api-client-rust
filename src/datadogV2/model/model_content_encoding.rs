// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentEncoding {
    #[serde(rename = "identity")]
    IDENTITY,
    #[serde(rename = "gzip")]
    GZIP,
    #[serde(rename = "deflate")]
    DEFLATE,
}

impl ToString for ContentEncoding {
    fn to_string(&self) -> String {
        match self {
            Self::IDENTITY => String::from("identity"),
            Self::GZIP => String::from("gzip"),
            Self::DEFLATE => String::from("deflate"),
        }
    }
}

impl Default for ContentEncoding {
    fn default() -> ContentEncoding {
        Self::IDENTITY
    }
}
