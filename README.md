# unifi-doorbell-notifier

[![test](https://github.com/sawadashota/unifi-doorbell-notifier/actions/workflows/test.yml/badge.svg)](https://github.com/sawadashota/unifi-doorbell-notifier/actions/workflows/test.yml)

Notify to your PC when Doorbell rung.

## Development Guide

```
$ cargo run -- init -c tmp/config.yaml

$ vi tmp/config.yaml
# edit config to connect Unifi Cloud key in your house

$ cargo run -- init -c tmp/config.yaml
```

Run test

```
cargo test
```

Build frontend

```
make web-build
```
