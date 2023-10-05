#!/bin/bash

# Build the plugin executable
cargo build --release

# Create the plugin tarball
cp target/release/spin-canary-install canary-install 
tar -czvf canary-install.tar.gz canary-install

# Create the plugin manifest
cat <<EOT > canary-install.json
{
    "name": "canary-install",
    "description": "A plugin to enable local AI development using cloud gpus.",
    "homepage": "https://developer.fermyon.com",
    "version": "0.1.0",
    "spinCompatibility": ">=1.4",
    "license": "Apache-2.0",
    "packages": [
        {
            "os": "macos",
            "arch": "aarch64",
            "url": "file:$(pwd)/canary-install.tar.gz",
            "sha256": "$(sha256sum canary-install.tar.gz | awk '{print $1;}')"
        }
    ]
}
EOT

# Cleanup
rm canary-install
