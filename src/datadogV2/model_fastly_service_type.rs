// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FastlyServiceType {
    #[serde(rename = "fastly-services")]
    FASTLY_SERVICES,
}

impl ToString for FastlyServiceType {
    fn to_string(&self) -> String {
        match self {
            Self::FASTLY_SERVICES => String::from("fastly-services"),
        }
    }
}

impl Default for FastlyServiceType {
    fn default() -> FastlyServiceType {
        Self::FASTLY_SERVICES
    }
}
