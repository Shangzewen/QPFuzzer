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

## Project Structure

Here's an overview of the project's main folders:

| Folder         | Description                                                                 |
|----------------|-----------------------------------------------------------------------------|
| `archive`      | Contains code related to archiving and managing data.                       |
| `common`       | Shared utilities and configurations used across the project.                |
| `docs`         | Documentation and related resources.                                        |
| `emulator`     | Code for emulating different hardware and software environments.            |
| `frametracer`  | Tools and libraries for tracing and analyzing execution frames.             |
| `fuzzer`       | Core fuzzing logic and algorithms.                                          |
| `hoedur`       | Implements the Hoedur multi-stream input system.                            |
| `hoedur-analyze` | Analysis tools for Hoedur execution data.                                 |
| `modeling`     | Models and simulations for various hardware and software components.        |
| `qemu-build`   | Build scripts and configurations for QEMU integration.                      |
| `qemu-rs`      | Rust bindings and extensions for QEMU.                                      |
| `qemu-sys`     | System-level patches and configurations for QEMU.                           |
| `reverse-eng`  | Tools and resources for reverse engineering tasks.                          |
| `scripts`      | Various scripts for testing, evaluation, and automation.                    |
| `target-zephyr`| Contains inputs and logs specific to the Zephyr target environment.         |

This table provides a brief description of the main components and their organization within the project.
