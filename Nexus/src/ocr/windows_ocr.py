from __future__ import annotations

from typing import Optional


def ocr_window(pid: int, tesseract_path: str | None = None, lang: str = "eng") -> Optional[str]:
    try:
        from pywinauto import Application
    except Exception:
        print("[nexus.ocr] pywinauto が見つかりません。OCRをスキップします。")
        return None

    try:
        import pytesseract
        from PIL import ImageGrab
    except Exception:
        print("[nexus.ocr] pytesseract/Pillow が見つかりません。OCRをスキップします。")
        return None

    try:
        if tesseract_path:
            import pytesseract as _pt
            _pt.pytesseract.tesseract_cmd = tesseract_path
        app = Application(backend="uia").connect(process=pid, timeout=3.0)
        top = app.top_window()
        rect = top.rectangle()
        bbox = (rect.left, rect.top, rect.right, rect.bottom)
        img = ImageGrab.grab(bbox=bbox)
        text = pytesseract.image_to_string(img, lang=lang or "eng")
        return text.strip() if text else None
    except Exception as e:
        print(f"[nexus.ocr] OCR失敗: pid={pid} error={e}")
        return None

