// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Set of tags to associate with your host.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostTags {
    /// Your host name.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// A list of tags to apply to the host.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl HostTags {
    pub fn new() -> HostTags {
        HostTags {
            host: None,
            tags: None,
        }
    }
}
