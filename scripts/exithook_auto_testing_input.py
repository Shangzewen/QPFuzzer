import subprocess
import os
import re
from os import listdir
from os.path import isfile, join
import time
from colorama import init, Fore, Style
import sys
import socket


def verify_log(log_list):
    # set wont contains any duplicate element
    return len(log_list) != len(set(log_list))

def runner(input_file_path, input_files, socket, ip, port):
    crash_log = []
    for f in input_files:
        print(f"{Fore.RED}Testing {f} {Style.RESET_ALL}")
        process1 = subprocess.Popen(
            ["./run-input.sh",input_file_path+f],
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,  # Python 3: auto-decode output to str
            bufsize=1   # Line-buffered
        )

        # log Crash message
        for line in process1.stdout:
            if "Result: Crash" in line or "Hit exit hook" in line:
                # process the string with the target parten
                print(line, end='')
                match_crash = re.search(r'(Crash\s*\{[^}]+\})', line)
                match_exithooh = re.search(r'(ExitHook\s*\{[^}]+\})',line)
                if match_crash:
                    crash_info = match_crash.group(1)
                    print(crash_info)
                    crash_log.append(crash_info)
                elif match_exithooh:
                    crash_info = match_exithooh.group(1)
                    print(crash_info)
                    crash_log.append(crash_info)
            # else:
            #     print("No crash info found.")
                # print(line, end='')  # already includes newline
        process1.wait()
        # wait for the u-fuzz engine to restart
        socket.sendto(b"Finished running",(ip_addr,udp_port))
        time.sleep(1)
    return crash_log

if __name__ == "__main__":
    # input_file_path = "./target-zephyr/meaningful_input_multi_version/v350/"
    ip_addr = "127.0.0.1"
    udp_port = 12000
    input_file_path = sys.argv[1]
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    # get the list of files in the target directory
    input_files = [f for f in listdir(input_file_path) if isfile(join(input_file_path, f))]
    print(input_files)
    crash_log = runner(input_file_path, input_files, client_socket,ip_addr,udp_port)
    result = verify_log(crash_log)
    print(f"{Fore.GREEN}Potential Crash : {result} {Style.RESET_ALL}")
    # Update the server side to change exploit
    client_socket.sendto(b"hello",(ip_addr,udp_port))
