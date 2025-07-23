import subprocess
import os
import re
from os import listdir
from os.path import isfile, join
import time
from colorama import init, Fore, Style
input_file_path = "./target-zephyr/meaningful_input_multi_version/v350/"
# get the list of files in the target directory
input_files = [f for f in listdir(input_file_path) if isfile(join(input_file_path, f))]
print(input_files)
crash_log = []

def verify_log(log_list):
    # set wont contains any duplicate element
    return len(log_list) != len(set(log_list))
# Run the docker exec command and stream output
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
        if "Result: Crash" in line or "Result: ExitHook" in line:
            # process the string with the target parten
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
            else:
                print("No crash info found.")
            # print(line, end='')  # already includes newline
    process1.wait()
    # wait for the u-fuzz engine to restart 
    time.sleep(3)
# print(crash_log)
result = verify_log(crash_log)
print(f"{Fore.GREEN}Potential Crash : {result} {Style.RESET_ALL}")
