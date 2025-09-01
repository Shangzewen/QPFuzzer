# aIRQFuzz (Zigbee) - QEMU Directed Firmware Protocol Fuzzer
An emulation-based, directed fuzzing framework that automatically discovers vulnerabilities deep into the wireless protocol implementation of bare-metal firmware. We evaluate aIRQFuzz on two distinct targets (BLE and Zigbee) to demonstrate both its effectiveness and its extensibility.  aIRQFuzz opens possibilities for emulation-based and stateful fuzzing of complex wireless protocols.

<p align="center">
  <img src="figs/Overview.png" alt="aIRQFuzz Overview and Design">
</p>

------

**Table of Contents**

1. [📋 Software Environment](#1--software-environment)
2. [⏩ Initial Compilation](#2--initial-compilation)
3. [🔀 Running Emulation Exploriation](#3--running-emulation-exploriation)
    * [Target Firmware Zigbee](#31-target-firmware-zigbee)
    * [Target Config Zigbee](#32-target-config-zigbee)
    * [Target Patch Zigbee](#33-target-patch-zigbee)
4. [🧑‍💻 Input Runner](#4--input-runner)
    * [Run single input](#41-run-single-input)
    * [Run single input with mmio/ram access documented](#42-run-single-input-with-mmio-ram-access-documented)
    * [Emulation input](#43-emulation-input)
5. [📄 Running the fuzzer](#5--running-the-fuzzer)
    * [Customised U-fuzz docker image](#51-customised-u-fuzz-docker-image)
    * [Running Totural](#52-running-totural)
6. [📄 Exploits](#6--exploits)
    * [Summary of potential Crashes](#61-summary-of-potential-crashes)
    * [Available Exploits](#62-available-exploits)
    * [Real board replication](#63-real-board-replication)
    * [Emulation replication](#64-emulation-replication)
7. [🧑‍💻 PoC script Auto-generator](#7--poc-script-auto-generator)
    * [Running Toturials and Potential Issues](#71-running-toturials-and-potential-issues)
8. [🧑‍💻 Auto-Weight calculator](#8--auto-weight-calculator)
    * [Running Toturials and Potential Issues](#81-running-toturials-and-potential-issues)
9. [📝 Citing aIRQFuzz](#9--citing-airqfuzz)



------

# 1. 📋 Software Environment
* **OS:** Ubuntu 24.04 - We recommend using Ubuntu 24.04 to build and run the emualtion engine for aIRQFuzz. As for the fuzzing engine, we prepared a ready-to-run docker [container](#51-customised-u-fuzz-docker-image) which build on ubuntu-18.04.  Alternativelly, you can refer to [U-fuzz]([url](https://github.com/asset-group/U-Fuzz/blob/main/README.md#2--initial-compilation)) github repo for environment setup to ensure the correct OS environment.

# 2. ⏩ Initial Compilation 
Several requirements need to be installed before compiling the project. An automated script for Ubuntu 24.04 is provided on `requirements.sh`. To compile from source, simply run the following commands:
```
$ Download the content from this github link:
https://anonymous.4open.science/r/AIRQFuzz_ZBE/

$ cd aIRQFuzz_zigbee

$ ./requirements.sh # Create a python virtual environment and Install all requirements to compile emulator from source 

$ cargo build # Compile all binaries. It may take around 15min. Go get a coffe!
```

# 3. 🔀 Running Emulation Exploriation
Before running the emulation engine three inputs need to be provided as follows:

1: Target Firmware Binary and Elf

2: Target configuration which specifies the memmory layout for the target firmware, the exithook function list and Interrupt Injection method.

3: Customised Patch for Target whcih contains the necessary patch for advancing the emulation to protocol code space.

After compiling the project with the correct software environment, please run the following command
```
$ cd qpfuzzer_zigbee

target fodler was specified in fuzz_zigbee.sh
$ ./fuzz_zigbee.sh
```
The emulation log would be saved in the target fodler under name log-fuzzing.txt
<!-- ## 3.1 Target Firmware BLE
[Target Fimware BLE](TODO)
## 3.2 Target Config BLE
## 3.3 Target Patch BLE -->
## 3.1 Target Firmware Zigbee
[Zigbee binary](./target-zigbee/zephyr.bin)
[Zigbee elf](./target-zigbee/zephyr.elf)
## 3.2 Target Config Zigbee
[Zigbee config](./target-zigbee/config.yml)
## 3.3 Target Patch Zigbee
[Zigbee patch](./target-zigbee/hook_ref.rs)
[Zigbee patch for fuzzing](./target-zigbee/hook.rs)

# 4. 🧑‍💻 Input Runner
Once meaningful emulation session has been done. a corpus archive file will be saved at target_folder/runs directory. After un-tar it, individual file could be retrieved.

<!-- See help for detials:
```
cargo run --bin hoedur-arm -- fuzz --help
``` -->

## 4.1 Run single input
The following cmd could be used to run single input
```
$ cd qpfuzzer_zigbee

$ ./run-input-zigbee.sh <input.bin>
```

## 4.2 Run single input with mmio/ram access documented
The following cmd could be used to run single input
```
$ cd qpfuzzer_zigbee

$ ./run-input-zigbee-detail.sh <input.bin>
```

## 4.3 Emulation input
[Individual input zigbee](./target-zigbee/meaningful_input/)

# 5. 📄 Running the fuzzer
## 5.1 Customised U-fuzz docker image
*Can pull from docker hub*
```
docker pull airqfuzz/u-fuzz-docker:aIRQFuzz
```
## 5.2 Running Totural
**Step1:**
*build the project (zigbee_realtime_fuzzer)*

```
Edit the CMakeLists.txt
$ Uncomments line:802, 811-815 
  (`set(ZIGBEE_SRC src/zigbee_realtime_fuzzer.cpp libs/shared_memory.c)`)
  (`add_executable(zigbee_realtime_fuzzer ${ZIGBEE_SRC} libs/profiling.c)`)
  (`target_link_libraries(zigbee_realtime_fuzzer PRIVATE ${MINIMAL_FUZZER_LIBS} viface)`)
  (`target_compile_options(zigbee_realtime_fuzzer PRIVATE -w -O0)`)
  (`target_compile_definitions(zigbee_realtime_fuzzer PRIVATE -DFUZZ_WIFI_AP)`)

$ Comments line: 806, 825-829 which were configured for Ble fuzzing
  (set(BLE_SRC src/ble_realtime_fuzzer.cpp libs/shared_memory.c))
  (add_executable(ble_realtime_fuzzer ${BLE_SRC} libs/profiling.c))
  (target_link_libraries(ble_realtime_fuzzer PRIVATE ${MINIMAL_FUZZER_LIBS} viface))
  (target_compile_options(ble_realtime_fuzzer PRIVATE -w -O0))
  (target_compile_definitions(ble_realtime_fuzzer PRIVATE -DFUZZ_WIFI_AP))

$ ./build.sh all

```
**Step2:**
*Update the fuzzing config if needed*
```
$ sudo nano /home/user/U-Fuzz/configs/zigbee_final_config.json
```
set enable_mutation == true and enable_optimization == true to enable stateful protocol aware fuzzing

**Step3:**
*Running the fuzzer*
```
$ cd /home/user/U-Fuzz

$ sudo bin/zigbee_realtime_fuzzer
```
The fuzzing probability, max fuzzing time and max iteration could also be updated in the config file. More detials could be found in [U-fuzz repo]([url](https://github.com/asset-group/U-Fuzz/))

**Step4:**
*Replay the emulation single input  at aIRQFuzz*
```
$ cd ~/qpfuzer_zigbee

$ ./run-input-loop.sh <input.bin>
```
[Potential input](#43-emulation-input) were provided
**potential cmd:**
```
./run-input-loop.sh ./target-zigbee/meaningful_input/input-4265661.bin
```
 
For fuzzing the emulation, the fuzzing engine needs to be conencted with the emulation engine to intercept the communication. [Zigbee patch for fuzzing](./target-zigbee/hook.rs) was required instead of 
[Zigbee patch](./target-zigbee/hook_ref.rs).


# 6. 📄 Exploits
## 6.1.  Summary of potential Crashes:
To this day, aIRQFUZZ has found 96 potential crashes in the BLE implementation of Zephyr OS across multiple versions and 27 potential crashes in Zephyr/Nordic Zigbee implementation. 
### QPF effectiveness to find/replicate crashes

| Protocol | Fw. Version                  | Unique Crash | # Mutations | Potential Crash | Board Replication |
|----------|------------------------------|--------------|-------------|-----------------|-------------------|
| **BLE**  | V2.2.99                      | 78           | ≤ 3         | 68              | 2 (CVE-2020-10061, CVE-2020-10069) |
|          | V2.5.1                       | 2            | ≤ 3         | 2               | 0                 |
|          | V3.5.99                      | 5            | ≤ 3         | 5               | 1                 |
|          | V3.7.1                       | 18           | ≤ 2         | 11              | 1 (duplicate to V3.5) |
|          | V4.0.0                       | 13           | ≤ 3         | 10              | 1 (duplicate to V3.5) |
| **Total**| All versions                 | 116          | ≤ 3         | 96              | 3                 |
| **Zigbee** | Nordic V2.9.99 + Zephyr OS V3.7.9 | 27   | ≤ 5         | 27              | NA                |


## 6.2. Available Exploits
| Vulnerability Name | Exploit |
| --- | --- |
| Zigbee Emulation Crash t2_1_3328 | [t2_1_3328.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t2_1_3328.cpp) |
| Zigbee Emulation Crash t3_6_512 | [t3_6_512.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t3_6_512.cpp) |
| Zigbee Emulation Crash t3_7_256 | [t3_7_256.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t3_7_256.cpp) |
| Zigbee Emulation Crash t4_1_1792 | [t4_1_1792.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t4_1_1792.cpp) |
| Zigbee Emulation Crash t4_1_2304 | [t4_1_2304.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t4_1_2304.cpp) |
| Zigbee Emulation Crash t4_1_3328 | [t4_1_3328.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t4_1_3328.cpp) |
| Zigbee Emulation Crash t4_2_2048 | [t4_2_2048.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t4_2_2048.cpp) |
| Zigbee Emulation Crash t4_2_3072 | [t4_2_3072.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t4_2_3072.cpp) |
| Zigbee Emulation Crash t4_3_1280 | [t4_3_1280.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t4_3_1280.cpp) |
| Zigbee Emulation Crash t4_3_1792 | [t4_3_1792.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t4_3_1792.cpp) |
| Zigbee Emulation Crash t4_3_2816 | [t4_3_2816.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t4_3_2816.cpp) |
| Zigbee Emulation Crash t4_4_2560 | [t4_4_2560.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t4_4_2560.cpp) |
| Zigbee Emulation Crash t5_1_1792 | [t5_1_1792.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_1_1792.cpp) |
| Zigbee Emulation Crash t5_1_2304 | [t5_1_2304.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_1_2304.cpp) |
| Zigbee Emulation Crash t5_1_3328 | [t5_1_3328.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_1_3328.cpp) |
| Zigbee Emulation Crash t5_2_1536 | [t5_2_1536.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_2_1536.cpp) |
| Zigbee Emulation Crash t5_2_2048 | [t5_2_2048.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_2_2048.cpp) |
| Zigbee Emulation Crash t5_2_3072 | [t5_2_3072.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_2_3072.cpp) |
| Zigbee Emulation Crash t5_3_1280 | [t5_3_1280.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_3_1280.cpp) |
| Zigbee Emulation Crash t5_3_1792 | [t5_3_1792.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_3_1792.cpp) |
| Zigbee Emulation Crash t5_3_2816 | [t5_3_2816.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_3_2816.cpp) |
| Zigbee Emulation Crash t5_4_1024 | [t5_4_1024.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_4_1024.cpp) |
| Zigbee Emulation Crash t5_4_1536 | [t5_4_1536.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_4_1536.cpp) |
| Zigbee Emulation Crash t5_4_2560 | [t5_4_2560.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_4_2560.cpp) |
| Zigbee Emulation Crash t5_5_768 | [t5_5_768.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_5_768.cpp) |
| Zigbee Emulation Crash t5_5_1280 | [t5_5_1280.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_5_1280.cpp) |
| Zigbee Emulation Crash t5_5_2304 | [t5_5_2304.cpp](./target-zigbee/zigbee_emulation_exploit/zigbee_emulation_report/t5_5_2304.cpp) |

<!-- ## 6.3. Real board replication
Our group used nrf52840DK board to verify the potential crash on the real board.
To launch such attack, please follow the attack tutorial that vakt-ble provided in section [4.1 Launching Sweyntooth Attacks](https://github.com/asset-group/vakt-ble-defender?tab=readme-ov-file#41-launching-sweyntooth-attacks)
### 6.3.1. Realboard crash script
TODO -->

## 6.3. Emulation replication
Emulation replication requires the auto-generated [PoC scripts](#62-available-exploits) running by the fuzzing engine to replay the crash sequence. Both the enable_mutation and enable_optimization need to be set to false to eliminate the normal mutation operation.
The detailed emulation replication [toturial](./toturial/emulation_replication_toturial.html) was provided.
<!-- Add a file for toturial -->

# 7. 🧑‍💻 PoC script Auto-generator
## 7.1 Running Toturials and Potential Issues

# 8. 🧑‍💻 Auto Weight calculator
## 8.1 Running Toturials and Potential Issues


# 9. 📝 Citing aIRQFuzz

```
Todo
```



<!-- 
Todo:
1. finished the fuzzing docker (separete ble and zigbee)
2. Update the ## 5.2 Running Totural
3. output the docker
4. realboard replication script
6. Finished the PoC generator part
7. Make sure the anonymouse
 -->