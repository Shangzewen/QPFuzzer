import subprocess
import threading
import os
import socket
import time

# Kill the previous running fuzzer
def stream_output(process, name):
    for line in process.stdout:
        print(f"[{name}] {line}", end='')  # tag output by source
    process.stdout.close()

def udp_listener(host='0.0.0.0', port=12000):
    global data_received
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind((host, port))
    # print(f"[UDP] Listening on {host}:{port}")
    while True:
        data, addr = sock.recvfrom(1024)
        data_received = data.decode()
        print(f"[UDP] Received from {addr}: {data.decode()}")


def get_files_in_container(container, path):
    cmd = f"docker exec {container} sh -c 'find {path} -maxdepth 1 -name *.cpp'"
    result = subprocess.run(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if result.returncode != 0:
        print(f"Error: {result.stderr}")
        return []
    path_file_list = [line.strip() for line in result.stdout.strip().split('\n') if line.strip()]
    file_list = []
    for ele in path_file_list:
        file_list.append((ele.split("/")[-1]).split(".")[0])
    print(file_list)
    return file_list
if __name__ == "__main__":
    counter_exploit = 0
    container_name = "strange_ellis"
    input_files = get_files_in_container(container_name, "/home/user/U-Fuzz/modules/exploits/ble")
    print(input_files)
    # input_file_path = sys.argv[1]
    # input_files = [f for f in listdir(input_file_path) if isfile(join(input_file_path, f))]
    exploit_name = input_files[counter_exploit]
    # input_file1 = "./target-zephyr/inputs/sm_pairing_rsp_good350.bin"
    cmd = "cd ./U-Fuzz && sudo bin/ble_realtime_fuzzer --exploit="+exploit_name
    data_received = ""
    thread = threading.Thread(target=udp_listener, daemon=True)
    thread.start()
    # Run the docker exec command and stream output
    while True:
        # Kill the running process if it is not closed properly
        print(data_received)
        if data_received != "":
            print(f"exploit needs to be updated")
            counter_exploit +=1
            exploit_name = input_files[counter_exploit]
            cmd = "cd ./U-Fuzz && sudo bin/ble_realtime_fuzzer --exploit="+exploit_name
            print(exploit_name)
            data_received = ""
        print("This is the exploit name"+exploit_name)
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
            # print(line, end='')  # already includes newline
            if "Crash detected" in line:
                # data, addr = server_socket.recvfrom(1024)
                print("Kill the process!!")
                time.sleep(2)
                process1.kill()
        process1.wait()
