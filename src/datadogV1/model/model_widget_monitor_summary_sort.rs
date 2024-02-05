// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetMonitorSummarySort {
    #[serde(rename = "name")]
    NAME,
    #[serde(rename = "group")]
    GROUP,
    #[serde(rename = "status")]
    STATUS,
    #[serde(rename = "tags")]
    TAGS,
    #[serde(rename = "triggered")]
    TRIGGERED,
    #[serde(rename = "group,asc")]
    GROUP_ASCENDING,
    #[serde(rename = "group,desc")]
    GROUP_DESCENDING,
    #[serde(rename = "name,asc")]
    NAME_ASCENDING,
    #[serde(rename = "name,desc")]
    NAME_DESCENDING,
    #[serde(rename = "status,asc")]
    STATUS_ASCENDING,
    #[serde(rename = "status,desc")]
    STATUS_DESCENDING,
    #[serde(rename = "tags,asc")]
    TAGS_ASCENDING,
    #[serde(rename = "tags,desc")]
    TAGS_DESCENDING,
    #[serde(rename = "triggered,asc")]
    TRIGGERED_ASCENDING,
    #[serde(rename = "triggered,desc")]
    TRIGGERED_DESCENDING,
    #[serde(rename = "priority,asc")]
    PRIORITY_ASCENDING,
    #[serde(rename = "priority,desc")]
    PRIORITY_DESCENDING,
}

impl ToString for WidgetMonitorSummarySort {
    fn to_string(&self) -> String {
        match self {
            Self::NAME => String::from("name"),
            Self::GROUP => String::from("group"),
            Self::STATUS => String::from("status"),
            Self::TAGS => String::from("tags"),
            Self::TRIGGERED => String::from("triggered"),
            Self::GROUP_ASCENDING => String::from("group,asc"),
            Self::GROUP_DESCENDING => String::from("group,desc"),
            Self::NAME_ASCENDING => String::from("name,asc"),
            Self::NAME_DESCENDING => String::from("name,desc"),
            Self::STATUS_ASCENDING => String::from("status,asc"),
            Self::STATUS_DESCENDING => String::from("status,desc"),
            Self::TAGS_ASCENDING => String::from("tags,asc"),
            Self::TAGS_DESCENDING => String::from("tags,desc"),
            Self::TRIGGERED_ASCENDING => String::from("triggered,asc"),
            Self::TRIGGERED_DESCENDING => String::from("triggered,desc"),
            Self::PRIORITY_ASCENDING => String::from("priority,asc"),
            Self::PRIORITY_DESCENDING => String::from("priority,desc"),
        }
    }
}
