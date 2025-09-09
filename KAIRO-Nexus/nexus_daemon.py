import time
import json
import os
import argparse
from datetime import datetime
from utils import send_to_ui_via_pid # Import the UI automation helper function

def load_maps(map_file):
    """Load the P-address to PID mappings."""
    with open(map_file, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    address_map = {}
    for binding in data.get('bindings', []):
        p_addr = binding.get('p')
        pid = binding.get('ui', {}).get('pid') # Get PID from 'ui' object
        if p_addr and pid is not None: # Check for None explicitly
            address_map[p_addr] = pid
    return address_map

def process_task_file(task_path, address_map):
    """Processes a single task JSON file."""
    print(f"[NEXUS] Processing task: {os.path.basename(task_path)}")
    try:
        with open(task_path, 'r', encoding='utf-8') as f:
            task = json.load(f)
    except json.JSONDecodeError as e:
        print(f"[NEXUS] Invalid JSON in {task_path}: {e}")
        return None # Indicates a non-fatal error, allowing rename

    from_p = task.get('from_p')
    to_p = task.get('to_p')
    body = task.get('body')

    if not all([from_p, to_p, body]):
        print(f"[NEXUS] Task file {task_path} is missing fields.")
        return None

    # Handle control commands
    if body.upper() == 'NEXUS-END':
        print("[NEXUS] NEXUS-END command received. Shutting down.")
        return "SHUTDOWN"

    # Get PID for the target P-address
    target_pid = address_map.get(to_p)
    
    # Special handling for 'cli' to launch a new PowerShell window
    if to_p == "cli" and target_pid == 0: # 0 is the special PID for launching new
        print(f"[NEXUS] Target 'cli' is set to launch a new PowerShell window.")
        # Call send_to_ui_via_pid with pid=0 to trigger launch logic
        send_result = send_to_ui_via_pid(0, body)
    elif not target_pid:
        print(f"[NEXUS] Could not find PID for target P-address '{to_p}' in maps.")
        return None
    else:
        # Normal case: send to existing PID
        print(f"[NEXUS] Sending content to UI for '{to_p}' (PID: {target_pid})...")
        send_result = send_to_ui_via_pid(target_pid, body)
    
    status = send_result.get("status", "error")
    message = send_result.get("message", "")
    captured_output = send_result.get("captured_output", "Output capture failed or not available.")

    if status == "ok":
        print(f"[NEXUS] Content sent successfully to '{to_p}'.")
        print(f"[NEXUS] Captured output: {captured_output[:100]}...") # Log captured output
    else:
        print(f"[NEXUS] Failed to send content to '{to_p}': {message}")

    # The "relaying response back to source" now means making it available in the log file
    task_result = {
        "from_p": from_p,
        "to_p": to_p,
        "command_body": body,
        "send_status": status,
        "send_message": message,
        "captured_output": captured_output 
    }
    print("[NEXUS] Task complete.")
    return json.dumps(task_result) # Return the result to be saved in the log file


def main_loop(maps_file, tasks_dir):
    """The main daemon loop."""
    print("[NEXUS] Nexus Daemon starting...")
    print(f"[NEXUS] Using map: {maps_file}")
    print(f"[NEXUS] Monitoring directory: {tasks_dir}")
    
    try:
        address_map = load_maps(maps_file)
    except Exception as e:
        print(f"[NEXUS] CRITICAL: Could not load map file: {e}")
        return

    while True:
        try:
            tasks = [f for f in os.listdir(tasks_dir) if f.endswith('.json')]
            for task_file in tasks:
                json_path = os.path.join(tasks_dir, task_file)
                processing_path = json_path.replace('.json', '.processing')
                
                os.rename(json_path, processing_path)
                
                result_data = process_task_file(processing_path, address_map)
                
                if result_data == "SHUTDOWN":
                    print("[NEXUS] Shutdown signal received. Exiting main loop.")
                    # Clean up the processing file before exiting
                    os.rename(processing_path, processing_path.replace('.processing', '.shutdown.log'))
                    return

                # Finalize by renaming to .log and include result data
                timestamp = datetime.now().strftime("%Y%m%d-%H%M%S")
                log_filename = f"{timestamp}_{task_file.replace('.json', '.log')}"
                log_path = os.path.join(tasks_dir, log_filename)
                
                try:
                    # Write the result data to the log file
                    with open(log_path, 'w', encoding='utf-8') as f:
                        if result_data:
                            f.write(result_data)
                        else:
                            f.write(json.dumps({"status": "error", "message": "Processing resulted in no data."}))
                    
                    # If write is successful, remove the processing file
                    os.remove(processing_path)
                    print(f"[NEXUS] Processed task saved to {os.path.basename(log_path)}")
                except Exception as e:
                    print(f"[NEXUS] FAILED to write log file for {os.path.basename(processing_path)}: {e}")
                    # Rename to .failed to prevent reprocessing
                    failed_path = processing_path.replace('.processing', '.failed')
                    os.rename(processing_path, failed_path)
                    print(f"[NEXUS] Renamed processing file to {os.path.basename(failed_path)}")

                print("-----")

            time.sleep(10) # Changed from 5 to 10 seconds
        except FileNotFoundError: 
            print(f"[NEXUS] ERROR: Tasks directory not found: {tasks_dir}. Retrying in 15s.")
            time.sleep(15)
        except Exception as e:
            print(f"[NEXUS] An error occurred in the main loop: {e}")
            time.sleep(10)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="KAIRO-Nexus Daemon: Monitors a folder for tasks.")
    parser.add_argument("--maps", default="./maps.json", help="Path to the maps.json file.")
    parser.add_argument("--tasks", default="./tasks", help="Path to the directory to monitor for tasks.")
    args = parser.parse_args()

    main_loop(args.maps, args.tasks)