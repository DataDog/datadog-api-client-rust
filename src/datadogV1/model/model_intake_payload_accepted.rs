// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The payload accepted for intake.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntakePayloadAccepted {
    /// The status of the intake payload.
    #[serde(rename = "status")]
    pub status: Option<String>,
}

impl IntakePayloadAccepted {
    pub fn new() -> IntakePayloadAccepted {
        IntakePayloadAccepted { status: None }
    }

    pub fn status(&mut self, value: String) -> &mut Self {
        self.status = Some(value);
        self
    }
}

impl Default for IntakePayloadAccepted {
    fn default() -> Self {
        Self::new()
    }
}
