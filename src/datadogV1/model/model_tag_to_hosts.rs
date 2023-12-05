// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// In this object, the key is the tag, the value is a list of host names that are reporting that tag.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TagToHosts {
    /// A list of tags to apply to the host.
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::HashMap<String, Vec<String>>>,
}

impl TagToHosts {
    pub fn new() -> TagToHosts {
        TagToHosts { tags: None }
    }
}