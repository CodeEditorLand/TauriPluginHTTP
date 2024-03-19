// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#[path = "src/scope.rs"]
#[allow(dead_code)]
mod scope;

const COMMANDS: &[&str] = &["fetch", "fetch_cancel", "fetch_send", "fetch_read_body"];

/// HTTP scope entry object definition.
#[allow(unused)]
#[derive(schemars::JsonSchema)]
#[serde(untagged)]
enum ScopeEntry {
    /// A URL that can be accessed by the webview when using the HTTP APIs.
    /// Wildcards can be used following the URL pattern standard.
    ///
    /// See [the URL Pattern spec](https://urlpattern.spec.whatwg.org/) for more information.
    ///
    /// Examples:
    ///
    /// - "https://*" : allows all HTTPS origin on port 443
    ///
    /// - "https://*:*" : allows all HTTPS origin on any port
    ///
    /// - "https://*.github.com/tauri-apps/tauri": allows any subdomain of "github.com" with the "tauri-apps/api" path
    ///
    /// - "https://myapi.service.com/users/*": allows access to any URLs that begins with "https://myapi.service.com/users/"
    Value(String),

    Object {
        /// A URL that can be accessed by the webview when using the HTTP APIs.
        /// Wildcards can be used following the URL pattern standard.
        ///
        /// See [the URL Pattern spec](https://urlpattern.spec.whatwg.org/) for more information.
        ///
        /// Examples:
        ///
        /// - "https://*" : allows all HTTPS origin on port 443
        ///
        /// - "https://*:*" : allows all HTTPS origin on any port
        ///
        /// - "https://*.github.com/tauri-apps/tauri": allows any subdomain of "github.com" with the "tauri-apps/api" path
        ///
        /// - "https://myapi.service.com/users/*": allows access to any URLs that begins with "https://myapi.service.com/users/"
        url: String,
    },
}

// ensure scope entry is up to date
impl From<ScopeEntry> for scope::Entry {
    fn from(value: ScopeEntry) -> Self {
        let url = match value {
            ScopeEntry::Value(url) => url,
            ScopeEntry::Object { url } => url,
        };

        scope::Entry {
            url: urlpattern::UrlPattern::parse(
                urlpattern::UrlPatternInit::parse_constructor_string::<regex::Regex>(&url, None)
                    .unwrap(),
            )
            .unwrap(),
        }
    }
}

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .global_api_script_path("./api-iife.js")
        .global_scope_schema(schemars::schema_for!(ScopeEntry))
        .build();
}
