// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// An individual scalar query to one of the basic Datadog data sources.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScalarQuery {
    MetricsScalarQuery(Box<crate::datadogV2::model::MetricsScalarQuery>),
    EventsScalarQuery(Box<crate::datadogV2::model::EventsScalarQuery>),
}
