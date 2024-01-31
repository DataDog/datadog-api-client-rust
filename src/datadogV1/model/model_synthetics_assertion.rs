// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Object describing the assertions type, their associated operator,
/// which property they apply, and upon which target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyntheticsAssertion {
    SyntheticsAssertionTarget(crate::datadogV1::model::SyntheticsAssertionTarget),
    SyntheticsAssertionJSONPathTarget(crate::datadogV1::model::SyntheticsAssertionJSONPathTarget),
    SyntheticsAssertionXPathTarget(crate::datadogV1::model::SyntheticsAssertionXPathTarget),
}
