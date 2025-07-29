import json
import os

base_path = "D:/Dev/KAIRO/users"

for i in range(1, 10):
    agent_dir = os.path.join(base_path, f"Agent{i}", "agent_configs")
    agent_file = os.path.join(agent_dir, f"Agent{i}.json")

    if os.path.exists(agent_file):
        try:
            with open(agent_file, 'r') as f:
                data = json.load(f)
                p_address = data.get("p_address")
                print(f"Agent{i}: {p_address}")
        except Exception as e:
            print(f"Agent{i}: Error reading file - {e}")
    else:
        print(f"Agent{i}: File not found or inaccessible")
