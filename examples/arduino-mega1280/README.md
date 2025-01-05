The `Mega1280` board cannot be auto-detected by ravedude, you need to specify the port explicitly.

This can be done in two ways:

- Add the `--port` flag to the cargo avr invocation, such as in the `.cargo/config.toml`, for example:

```toml
runner = "cargo avr run board mega1280 --baudrate 57600 --port /dev/ttyUSB0"
```

- Set the `CARGO_AVR_PROGRAMMER_PORT` environment variable, for example:

```bash
CARGO_AVR_PROGRAMMER_PORT=/dev/ttyUSB0 cargo run --bin mega1280-i2cdetect
```
