from nicegui import ui
import subprocess
import threading
import re
import os
from ansi2html import Ansi2HTMLConverter
import asyncio

# Global variable
process = None
ble_state_visible = False
# function to show the ble statemachine
def show_ble_state():
    ui.notify('BLE it is!')
    global ble_state_visible
    ble_state_visible = not ble_state_visible  # Toggle visibility
    ble_state_image.set_visibility(ble_state_visible)
# Function to update html code to set color
def set_color_html(str_html):
    update_str1 = str_html.replace('class' ,'style')
    update_str2 = update_str1.replace('ansi36', 'color: blue;')
    update_str3 = update_str2.replace('ansi32', 'color: red;')
    update_str4 = update_str3.replace('ansi33', 'color: purple;')
    return update_str4
def remove_ansi_codes(text):
    ansi_escape = re.compile(r'\x1B\[[0-?9;]*[mK]')
    return ansi_escape.sub('', text)
# will run a command and update the output_widget for the terminal output
def run_terminal_command(command, output_markdown1):
    global process
    process = subprocess.Popen(command,stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,cwd='/home/asset/qpfuzzer_ble')
    conv = Ansi2HTMLConverter()
    
    # for line in process.stdout:
        # output_markdown.set_content(output_markdown.content + line.strip() + '\n')  # Append new output
    for line in process.stdout:
        if 'RX' in line and 'PKT' not in line:
            # clean_line = remove_ansi_codes(line.strip())
            clean_line = conv.convert(line, full=False)
            # print(clean_line)
            update_html = set_color_html(clean_line)
            # colored_line = conv.convert(clean_line,full=False)
            output_markdown1.set_content(output_markdown1.content + '<br>' +update_html + '<br>')  # Append new output
            # output_markdown1.style('color: red')
            output_markdown1.update()
        elif 'TX' in line and 'PKT' not in line:
            # clean_line = remove_ansi_codes(line.strip())
            clean_line = conv.convert(line, full=False)
            # print(clean_line)
            update_html = set_color_html(clean_line)
            # colored_line = conv.convert(clean_line,full=False)
            output_markdown1.set_content(output_markdown1.content + '<br>' +update_html + '<br>')  # Append new output
            # output_markdown1.style('color: green')
            output_markdown1.update()
        else:
            continue

async def fuzz():
    notifyer = ui.notification(timeout=None)
    command = ['/bin/bash','fuzz.sh']
    threading.Thread(target=run_terminal_command, args=(command, output_markdown1), daemon=True).start()
    notifyer.spinner = True
    notifyer.message = f'Starting Fuzzing Process'
    await asyncio.sleep(5)
    notifyer.spinner = False
    notifyer.message = 'Fuzzing Started'
    await asyncio.sleep(2)
    notifyer.dismiss()

# Function to stop the terminal command
async def on_stop_command():
    global process
    notifyer = ui.notification(timeout=None)
    if process is not None:
        try:
            process.terminate()  # Attempt to terminate the subprocess
            os.system('pkill -f hoedur-arm')
            notifyer.message = 'Fuzzing Stopped'
            await asyncio.sleep(2)
            notifyer.dismiss()
            process.wait(timeout=2)  # Wait for up to 5 seconds for the process to terminate
        except subprocess.TimeoutExpired:
            process.kill()  # If it doesn't terminate, forcefully kill it
            process.wait()  # Wait for it to be killed
        finally:
            process = None  # Reset the process reference

with ui.row().classes('w-full justify-center items-center'):
    # with ui.card().classes('flex items-center justify-center').tight():
    ui.image('/home/asset/qpfuzzer_ble/docs/logo.png').style('max-width:5%; max-height:5%;')
    ui.label('QPFuzzer').style('color: green ; font-size: 36px; font-weight: bold;')
    with ui.dropdown_button(auto_close=True).props('outline round').classes('shadow-lg items-center justify-center'):
    # with ui.dropdown_button('QPFuzzer', auto_close=True,color='green').style('font-size: 18px;'):
        ui.item('Bluetooth Low Energy', on_click=show_ble_state)
        ui.item('Zigbee', on_click=lambda: ui.notify('Zigbee it is!'))

with ui.column().classes('h-screen w-full items-center gap-4'):
    with ui.row().classes('gap-4 m-11'):
        with ui.column().classes("items-center justify-center"):
            ui.label('Fuzzing Session').style('color: red ; font-size: 18px; font-weight: bold;')
            with ui.card().classes('w-[45vw] h-[70vh] flex items-center justify-center'):
                output_markdown1 = ui.html("").classes('w-full h-full overflow-y-auto flex-grow')  # Using markdown for output
                output_markdown1.style('font-size: 18px;') 

            # add the fuzz button to start the fuzzing process
            with ui.row().classes('items-center gap-4'):
                ui.button('Fuzz', on_click=fuzz)
                ui.button('Stop', on_click=on_stop_command)
        with ui.column().classes("items-center justify-center"):
            ui.label('State Machine').style('color: red ; font-size: 18px; font-weight: bold;')
            with ui.card().classes('w-[45vw] h-[70vh] flex items-center justify-center'):
                ble_state_image = ui.image('/home/asset/qpfuzzer/qpfuzzer_gui/ble_state_machine.jpeg').style('max-width:55%; max-height:100%;')
                ble_state_image.set_visibility(ble_state_visible)  # Bind visibility to state
# ui.markdown().style()
ui.run()