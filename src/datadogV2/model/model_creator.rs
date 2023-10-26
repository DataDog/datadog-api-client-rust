// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Creator {
    /// Email of the creator.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Handle of the creator.
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Name of the creator.
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
}

impl Creator {
    /// Creator of the object.
    pub fn new() -> Creator {
        Creator {
            email: None,
            handle: None,
            name: None,
        }
    }
}
