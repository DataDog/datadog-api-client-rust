// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
#[repr(i64)]
pub enum SyntheticsPlayingTab {
    MAIN_TAB = -1,
    NEW_TAB = 0,
    TAB_1 = 1,
    TAB_2 = 2,
    TAB_3 = 3,
}

impl ToString for SyntheticsPlayingTab {
    fn to_string(&self) -> String {
        match self {
            Self::MAIN_TAB => String::from("-1"),
            Self::NEW_TAB => String::from("0"),
            Self::TAB_1 => String::from("1"),
            Self::TAB_2 => String::from("2"),
            Self::TAB_3 => String::from("3"),
        }
    }
}
