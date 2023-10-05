use std::collections::HashMap;

// Define the URLs for the canary plugins
pub fn create_url_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert(
        "cloud",
        "https://github.com/fermyon/cloud-plugin/releases/download/canary/cloud.json",
    );
    m.insert(
        "cloud-gpu",
        "https://github.com/fermyon/spin-cloud-gpu/releases/download/canary/cloud-gpu.json",
    );
    m.insert(
        "js2wasm",
        "https://github.com/fermyon/spin-js-sdk/releases/download/canary/js2wasm.json",
    );
    m.insert(
        "py2wasm",
        "https://github.com/fermyon/spin-python-sdk/releases/download/canary/py2wasm.json",
    );
    m.insert(
        "pluginify",
        "https://github.com/itowlson/spin-pluginify/releases/download/canary/pluginify.json",
    );
    m.insert(
        "canary-install",
        "https://github.com/karthik2804/spin-canary-install/releases/download/canary/canary-install.json",
    );
    m
}
