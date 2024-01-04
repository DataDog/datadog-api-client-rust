// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncidentAttachmentAttachmentType {
    #[serde(rename = "link")]
    LINK,
    #[serde(rename = "postmortem")]
    POSTMORTEM,
}

impl ToString for IncidentAttachmentAttachmentType {
    fn to_string(&self) -> String {
        match self {
            Self::LINK => String::from("link"),
            Self::POSTMORTEM => String::from("postmortem"),
        }
    }
}
