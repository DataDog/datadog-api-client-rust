// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IntakePayloadAccepted {
    /// A list of errors.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

impl IntakePayloadAccepted {
    /// The payload accepted for intake.
    pub fn new() -> IntakePayloadAccepted {
        IntakePayloadAccepted { errors: None }
    }
}
