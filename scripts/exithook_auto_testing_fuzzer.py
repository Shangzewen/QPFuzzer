import subprocess
import threading
import os
import socket
import time
from colorama import Fore, Style

def udp_listener(host='0.0.0.0', port=12000):
    global data_received, process1, exploit_name,cmd, input_files, counter_exploit
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind((host, port))
    # print(f"[UDP] Listening on {host}:{port}")
    while counter_exploit < len(input_files):
        data, addr = sock.recvfrom(1024)
        data_received = data.decode()
        print(f"[UDP] Received from {addr}: {data.decode()}")
        if data_received == "hello":
            # data_event1.set()
            counter_exploit +=1
            if counter_exploit< len(input_files):
                exploit_name = input_files[counter_exploit]
                cmd = "cd ./U-Fuzz && sudo bin/ble_realtime_fuzzer --exploit="+exploit_name
                print("This is the exploit running: "+exploit_name)
                process1.kill()
            else:
                process1.kill()
                print("Finished running script")
                break

        elif data_received == "Finished running":
            # data_event2.set()
            if process1:
                print(f"{Fore.RED}Lets kill process1{Style.RESET_ALL}")
                process1.kill()
    print("Finished all Exploit")

# output target exploit list from the dir in the continer
def get_files_in_container(container, path):
    cmd = f"docker exec {container} sh -c 'find {path} -maxdepth 1 -name *.cpp'"
    result = subprocess.run(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if result.returncode != 0:
        print(f"Error: {result.stderr}")
        return []
    path_file_list = [line.strip() for line in result.stdout.strip().split('\n') if line.strip()]
    file_list = []
    file_dictionary = {}
    for idx, ele in enumerate(path_file_list):
        file_list.append((ele.split("/")[-1]).split(".")[0])
        file_dictionary[idx] = (ele.split("/")[-1]).split(".")[0]
    # print(file_list)
    print(f"{Fore.RED}This is the exploit dictonary: {Style.RESET_ALL}",file_dictionary)
    return file_list


if __name__ == "__main__":
    # set two threading event to control switch of exploit and restart of fuzzer
    global process1, exploit_name, cmd, input_files, counter_exploit
    counter_exploit = 0
    container_name = "strange_ellis"
    # file list conatins all target exploits
    input_files = get_files_in_container(container_name, "/home/user/U-Fuzz/modules/exploits/ble_v350")
    print(f"{Fore.RED}This is the exploit list: {Style.RESET_ALL}",input_files)
    exploit_name = input_files[counter_exploit]
    cmd = "cd ./U-Fuzz && sudo bin/ble_realtime_fuzzer --exploit="+exploit_name
    data_received = ""
    # start the udp listener in parallel 
    thread_udp = threading.Thread(target=udp_listener, daemon=True)
    thread_udp.start()
    # Run the docker exec command and stream output
    while True:
        print(data_received)
        print(f"{Fore.GREEN}Cmd Running{Style.RESET_ALL}: " + cmd)
        print("This is the exploit name"+exploit_name)
        # Kill the running process if it is not closed properly
        os.system("docker exec -i "+ container_name +" sh -c 'sudo pkill main_thread'")
        process1 = subprocess.Popen(
            ["docker", "exec", "-i", container_name, "sh", "-c", cmd],
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,  # Python 3: auto-decode output to str
            bufsize=1   # Line-buffered
        )

        # # Print each line as it's received for debugging
        # for line in process1.stdout:
        #     print(line, end='')  # already includes newline
        process1.wait()
        # time.sleep(2)
