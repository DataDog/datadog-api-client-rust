// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RUMApplicationType {
    #[serde(rename = "rum_application")]
    RUM_APPLICATION,
}

impl ToString for RUMApplicationType {
    fn to_string(&self) -> String {
        match self {
            Self::RUM_APPLICATION => String::from("rum_application"),
        }
    }
}
