import os 
import subprocess
input_dir = '/home/asset/qpfuzzer/target-zephyr/runs/input'
input_files = os.listdir(input_dir)
# command_template = "cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- --config  $(pwd)/target-zephyr/config.yml --debug --trace --hook $(pwd)/target-zephyr/hook.rs run $(pwd)/target-zephyr/runs/input/{}"
command_template = "source ~/.bashrc && ./run-postinput.sh $(pwd)/target-zephyr/runs/input/{}"
# print(name_lst)
for input_file in input_files:
    # Construct the command
    command = command_template.format(input_file)
    # print(command)
    
    # Run the command and capture the output
    # process = subprocess.Popen(command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    process = subprocess.Popen(f"bash -l -c '{command}'", shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    stdout, stderr = process.communicate()
    
    # Construct the output file path in the input file's directory
    output_file = os.path.join(input_dir, f"{input_file}_output.txt")
    
    # Write the standard output to the output file
    with open(output_file, 'w') as f:
        f.write(stdout)
    
    # (Optional) Write standard error to the output file if needed
    if stderr:
        with open(output_file, 'a') as f:  # Append to the same output file
            f.write("\n\nStandard Error:\n")
            f.write(stderr)

print("Commands executed and outputs saved to respective files.")