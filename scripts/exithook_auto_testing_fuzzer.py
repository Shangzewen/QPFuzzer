import subprocess
import threading
import os
container_name = "strange_ellis"
exploit_name = "crash6_excep5_pc_2676e"
# input_file1 = "./target-zephyr/inputs/sm_pairing_rsp_good350.bin"
cmd = "cd ./U-Fuzz && sudo bin/ble_realtime_fuzzer --exploit="+exploit_name
# Kill the previous running fuzzer
def stream_output(process, name):
    for line in process.stdout:
        print(f"[{name}] {line}", end='')  # tag output by source
    process.stdout.close()
# Run the docker exec command and stream output

while True:
    # Kill the running process if it is not closed properly
    os.system("docker exec -i "+ container_name +" sh -c 'sudo pkill main_thread'")
    process1 = subprocess.Popen(
        ["docker", "exec", "-i", container_name, "sh", "-c", cmd],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,  # Python 3: auto-decode output to str
        bufsize=1   # Line-buffered
    )
    # os.system("./run-input.sh "+ input_file1)

    # # Print each line as it's received
    for line in process1.stdout:
        print(line, end='')  # already includes newline
        if "Crash detected" in line:
            print("Kill the process!!")
            process1.kill()
    process1.wait()

# Run the docker exec command and stream output
# process2 = subprocess.Popen(
#     ["./run-input.sh", input_file1],
#     stdout=subprocess.PIPE,
#     stderr=subprocess.STDOUT,
#     text=True,  # Python 3: auto-decode output to str
#     bufsize=1   # Line-buffered
# )
# Create threads to monitor both
# t1 = threading.Thread(target=stream_output, args=(process1, "Fuzzer"))
# t2 = threading.Thread(target=stream_output, args=(process2, "Runner"))

# Start threads
# t1.start()
# t2.start()

# # Wait for both to finish
# t1.join()
# t2.join()

# # Optionally get return codes
# print(f"Fuzzer exited with {process1.wait()}")
# print(f"Runner exited with {process2.wait()}")
# os.system("docker exec -i "+ container_name +" sh -c 'sudo lsof -i:9000'")
# print("\nExited with code:", process1.returncode)