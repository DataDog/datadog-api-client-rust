// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// An object related to a user.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserResponseIncludedItem {
    Organization(Box<crate::datadogV2::model::Organization>),
    Permission(Box<crate::datadogV2::model::Permission>),
    Role(Box<crate::datadogV2::model::Role>),
}