// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsPlayingTab {
    MAIN_TAB,
    NEW_TAB,
    TAB_1,
    TAB_2,
    TAB_3,
}

impl Serialize for SyntheticsPlayingTab {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(match self {
            SyntheticsPlayingTab::MAIN_TAB => -1,
            SyntheticsPlayingTab::NEW_TAB => 0,
            SyntheticsPlayingTab::TAB_1 => 1,
            SyntheticsPlayingTab::TAB_2 => 2,
            SyntheticsPlayingTab::TAB_3 => 3,
        })
    }
}

impl<'de> Deserialize<'de> for SyntheticsPlayingTab {
    fn deserialize<D>(deserializer: D) -> Result<SyntheticsPlayingTab, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i64 = i64::deserialize(deserializer)?;
        Ok(match s {
            -1 => SyntheticsPlayingTab::MAIN_TAB,
            0 => SyntheticsPlayingTab::NEW_TAB,
            1 => SyntheticsPlayingTab::TAB_1,
            2 => SyntheticsPlayingTab::TAB_2,
            3 => SyntheticsPlayingTab::TAB_3,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsPlayingTab: {}",
                    s
                )))
            }
        })
    }
}
