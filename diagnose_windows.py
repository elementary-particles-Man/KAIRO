from pywinauto import Desktop

print("--- Diagnosing with UIA backend ---")
try:
    windows_uia = Desktop(backend='uia').windows()
    for w in windows_uia:
        print(f"UIA Window: ''{w.window_text()}'' (PID: {w.process_id()})")
except Exception as e:
    print(f"Could not get windows with UIA backend: {e}")

print("\n--- Diagnosing with Win32 backend ---")
try:
    windows_win32 = Desktop(backend='win32').windows()
    for w in windows_win32:
        print(f"Win32 Window: ''{w.window_text()}'' (PID: {w.process_id()})")
except Exception as e:
    print(f"Could not get windows with Win32 backend: {e}")

