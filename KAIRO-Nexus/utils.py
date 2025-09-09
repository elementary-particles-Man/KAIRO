import json
import time
import pyautogui
from pywinauto import Application, Desktop # Assuming pywinauto is installed
import psutil # To get process info
import subprocess # For launching new processes
import tempfile
import os
import traceback

import traceback

def launch_and_connect_powershell():
    """
    Launches a new PowerShell window and connects to it using pywinauto.
    Returns the pywinauto window object.
    """
    print("Launching new PowerShell window...")
    # Start PowerShell process
    # Use creationflags=subprocess.CREATE_NEW_CONSOLE to ensure a new window
    process = subprocess.Popen(["powershell.exe"], creationflags=subprocess.CREATE_NEW_CONSOLE)
    time.sleep(2) # Give PowerShell time to start and create its window

    try:
        # Connect to the newly launched PowerShell window
        app = Application(backend="uia").connect(class_name="ConsoleWindowClass", process=process.pid)
        main_window = app.top_window()
        print(f"Successfully connected to new PowerShell window (PID: {process.pid}).")
        return main_window
    except Exception as e:
        print(f"Failed to connect to new PowerShell window: {e}")
        # Terminate the process if connection failed
        process.terminate()
        raise

def send_to_ui_via_pid(pid, content):
    """
    Sends content to a UI application identified by PID using pywinauto and pyautogui.
    If pid is 0 (or a special value), it launches a new PowerShell window.
    Output is captured via a temporary file.
    """
    output_file = None
    try:
        app = None
        main_window = None

        if pid == 0: # Special PID to indicate launching a new PowerShell window
            main_window = launch_and_connect_powershell()
        else:
            # Connect to the window using the win32 backend, which was found to be more reliable.
            main_window = None
            try:
                print(f"Attempting to find window for PID: {pid} with backend 'win32'")
                desktop = Desktop(backend="win32")
                windows = desktop.windows(process=pid)
                if not windows:
                    raise Exception(f"No windows found for PID {pid} with backend 'win32'")
                
                # Heuristic: Find the most likely candidate window.
                # Often, the main window is visible and has a title (even if empty).
                for w in windows:
                    if w.is_visible():
                        main_window = w
                        break
                
                if not main_window:
                    # If no visible window, take the first one.
                    main_window = windows[0]

                print(f"Successfully found window for PID: {pid} -> {main_window}")

            except Exception as e:
                raise Exception(f"Failed to connect to window for PID {pid}: {e}")

        # --- Create a temporary file for output redirection ---
        output_fd, output_path = tempfile.mkstemp(text=True, suffix=".txt")
        os.close(output_fd)
        output_file = output_path
        
        # Construct the command to execute, redirecting output to the temp file
        # Note: This assumes the 'content' is a command that can be redirected.
        # This might need adjustment depending on what 'content' actually is.
        # For PowerShell, we can use Out-File.
        # Let's create a small script to execute the command and capture output.
        
        script_content = f"""
$ErrorActionPreference = "Stop"
try {{
    {content} | Out-File -FilePath "{output_path}" -Encoding utf8 -NoNewline
}} catch {{
    $_ | Out-File -FilePath "{output_path}" -Encoding utf8 -NoNewline
}}
"""
        
        # Proceed with UI automation if a window was found
        main_window.set_focus()
        time.sleep(1) # Reduced sleep
        
        # Use pyperclip to paste the script. It's more reliable for multi-line content.
        import pyperclip
        pyperclip.copy(script_content)
        
        pyautogui.hotkey('ctrl', 'v')
        time.sleep(1) # Reduced sleep
        
        pyautogui.press('enter')
        
        # --- Wait for output and capture it ---
        # The wait time is still tricky. We'll wait for a bit and then check the file.
        # A more advanced solution would involve checking for a "sentinel" value at the end of the output.
        time.sleep(10) # Wait for the command to execute. This is still a guess.
        
        captured_output = ""
        try:
            with open(output_path, 'r', encoding='utf-8') as f:
                captured_output = f.read()
        except FileNotFoundError:
            captured_output = "Error: Output file not found."
        except Exception as read_e:
            captured_output = f"Error reading output file: {read_e}"
            
        return {"status": "ok", "message": f"Content sent to PID {pid}.", "captured_output": captured_output}
    except Exception as e:
        error_message = f"Failed to send content to PID {pid}: {str(e)}"
        tb = traceback.format_exc()
        return {"status": "error", "message": f"{error_message}\n{tb}", "captured_output": ""}
    finally:
        # --- Clean up the temporary file ---
        if output_file and os.path.exists(output_file):
            os.remove(output_file)
