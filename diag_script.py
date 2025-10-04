import sys
sys.path.insert(0, 'G:/マイドライブ/Develop/KAIRO')
from pywinauto import Application

try:
    pid = 5484 # CLI PID
    app = Application(backend="uia").connect(process=pid, timeout=5)
    top = app.top_window()
    top.set_focus()
    print("--- Top Window ---")
    print(f"Title: {top.window_text()}, Control Type: {top.friendly_class_name()}")
    print("\n--- All Descendant Controls ---")
    controls = top.descendants()
    for i, ctrl in enumerate(controls):
        try:
            text = ctrl.window_text()
            ctype = ctrl.friendly_class_name()
            print(f"{i}: Text='{text}', Type='{ctype}'")
        except Exception:
            print(f"{i}: Could not get info for control.")
    print("\n--- End of Controls ---")
except Exception as e:
    print(f"An error occurred: {e}")
