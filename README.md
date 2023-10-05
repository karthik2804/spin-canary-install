## Spin Canary Install Plugin

It is a simple [Spin](https://github.com/fermyon/spin) plugin to help install the canary version of plugins without having to specify URLs.

The currently supported list of plugins are cloud, cloud-gpu, pluginify, js2wasm, py2wasm and canary-install

### Installation

To install for the first time, use the following command:

```
spin plugins install -u https://github.com/karthik2804/spin-canary-install/releases/download/canary/canary-install.json -y
```

### Usage

```bash
spin canary-install <plugin-name>
```

### Upgrading the Plugin

```bash
spin canary-install canary-install
```