// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = include_str!("../README.md")]

/// Shared client configuration and authentication types.
pub mod datadog;
/// Types and API clients for Datadog API v1.
pub mod datadogV1;
pub mod datadogV2;
