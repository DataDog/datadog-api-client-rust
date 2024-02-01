// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Dynamic fields for which selections can be made, with field names as keys.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IncidentFieldAttributes {
    IncidentFieldAttributesSingleValue(
        Box<crate::datadogV2::model::IncidentFieldAttributesSingleValue>,
    ),
    IncidentFieldAttributesMultipleValue(
        Box<crate::datadogV2::model::IncidentFieldAttributesMultipleValue>,
    ),
}
