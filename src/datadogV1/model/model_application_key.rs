// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An application key with its associated metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKey {
    /// Hash of an application key.
    #[serde(rename = "hash")]
    pub hash: Option<String>,
    /// Name of an application key.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Owner of an application key.
    #[serde(rename = "owner")]
    pub owner: Option<String>,
}

impl ApplicationKey {
    pub fn new() -> ApplicationKey {
        ApplicationKey {
            hash: None,
            name: None,
            owner: None,
        }
    }
}
