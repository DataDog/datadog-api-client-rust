// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Possible Container models.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerItem {
    Container(Box<crate::datadogV2::model::Container>),
    ContainerGroup(Box<crate::datadogV2::model::ContainerGroup>),
}