// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Top list widget display options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToplistWidgetDisplay {
    ToplistWidgetStacked(Box<crate::datadogV1::model::ToplistWidgetStacked>),
    ToplistWidgetFlat(Box<crate::datadogV1::model::ToplistWidgetFlat>),
}
