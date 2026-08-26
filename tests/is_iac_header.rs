// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

// Verifies that Configuration::set_is_iac(true) causes the
// `X-Datadog-Managed-By: iac` header to be attached to outgoing requests,
// and that it is absent by default / when explicitly disabled.
//
// The header is baked into the client's default headers once, at
// AuthenticationAPI::with_config() time (see api.j2), rather than being
// re-inserted on every request. A local TCP listener stands in for the
// Datadog API so the test observes the real with_config() client, not a
// hand-rolled stand-in for it.

use datadog_api_client::datadog::Configuration;
use datadog_api_client::datadogV1::api_authentication::AuthenticationAPI;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn captured_header(is_iac: bool) -> Option<String> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let server = std::thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(&stream);
        let mut header_value = None;
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            if line == "\r\n" || line.is_empty() {
                break;
            }
            if let Some((name, value)) = line.split_once(':') {
                if name.eq_ignore_ascii_case("x-datadog-managed-by") {
                    header_value = Some(value.trim().to_string());
                }
            }
        }
        let mut stream = stream;
        let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n");
        header_value
    });

    let mut config = Configuration::default();
    config.set_is_iac(is_iac);
    config.server_index = 1; // the "{protocol}://{name}" server variant
    config
        .server_variables
        .insert("protocol".to_string(), "http".to_string());
    config
        .server_variables
        .insert("name".to_string(), format!("127.0.0.1:{port}"));

    let api = AuthenticationAPI::with_config(config);
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _ = rt.block_on(api.validate());

    server.join().unwrap()
}

#[test]
fn does_not_send_iac_header_by_default() {
    assert_eq!(captured_header(false), None);
}

#[test]
fn sends_iac_header_when_enabled() {
    assert_eq!(captured_header(true).as_deref(), Some("iac"));
}
