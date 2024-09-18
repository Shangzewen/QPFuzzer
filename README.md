# QPFuzzer - QEMU Directed Firmware Protocol Fuzzer

QPFuzzer is a directed firmware fuzzing implementation which utilizes a Hoedur multi-stream input to fuzz firmware employing complex protocols stacks such as BLE, Zigbee, LoRa, etc.

<p align="center">
  <img src="./docs/logo.png" alt="overview" width="300" height="auto"/>
</p>

## Getting Started

### Dependencies
```bash
./requirements.sh
```

## Fuzzer / Input Runner Commands

### Fuzzer

Basic usage:
```bash
./fuzz.sh
```

See help for details:
```bash
cargo run --bin hoedur-arm -- fuzz --help
```

### Input Runner

Run single input:
```bash
./run-input.sh target-zephyr/inputs/input-adv.bin
```

Run single post input (you can change initial input inside the script):

```bash
./run-postinput.sh target-zephyr/inputs/postinput-anchor-point.bin
```

See help for details:

```bash
cargo run --bin hoedur-arm -- fuzz --help
```

#### Debug Firmware

Set environment variable `GDB=1` before running any command:

```
GDB=1 ./run-input.sh target-zephyr/inputs/input-adv.bin
```

In another terminal, open gdb-multiarch:

```bash
gdb-multiarch
```

### Coverage

Run fuzzer with `./fuzz.sh` first.

Collect coverage report from corpus archive:
```bash
./coverage.sh
```

### Tracing / Hook Scripts

Trace logs are saved to `target-zephyr/log-tracing.txt`

```bash
./trace-input.sh target-zephyr/inputs/input-adv.bin
```
