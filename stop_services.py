import subprocess

PROCESS_NAMES = ["kairo-daemon.exe", "seed_node.exe", "mesh_node.exe"]

def stop_processes():
    for name in PROCESS_NAMES:
        try:
            subprocess.run(
                ["taskkill", "/F", "/IM", name],
                check=True,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL
            )
            print(f"{name} stopped")
        except subprocess.CalledProcessError:
            print(f"{name} not found or already stopped")
        except FileNotFoundError:
            print("taskkill command not found. Make sure you're on Windows.")
            break

if __name__ == "__main__":
    stop_processes()
