// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The payload accepted for intake.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntakePayloadAccepted {
    /// A list of errors.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<String>>,
}

impl IntakePayloadAccepted {
    pub fn new() -> IntakePayloadAccepted {
        IntakePayloadAccepted { errors: None }
    }

    pub fn errors(&mut self, value: Vec<String>) -> &mut Self {
        self.errors = Some(value);
        self
    }
}

impl Default for IntakePayloadAccepted {
    fn default() -> Self {
        Self::new()
    }
}
