"""Launch the packaged GUI on Windows and verify its control channel and screenshot."""
import json
from pathlib import Path
import socket
import subprocess
import time

root = Path(__file__).resolve().parents[2]
app = root / 'dist/release/Savol Office.exe'
process = subprocess.Popen([str(app), '--sample', 'sales', '--control', '17987'])
try:
    deadline = time.monotonic() + 30
    while True:
        if process.poll() is not None:
            raise RuntimeError(f'Savol Office exited at startup: {process.returncode}')
        try:
            connection = socket.create_connection(('127.0.0.1', 17987), timeout=2)
            break
        except OSError:
            if time.monotonic() >= deadline:
                raise
            time.sleep(.5)
    connection.settimeout(30)
    with connection, connection.makefile('rwb') as stream:
        for method, params in [
            ('ui.inspect', {}),
            ('cell.set', {'cell': 'A12', 'input': 'Русский Ёё — 123 456 ₽ — 中文 繁體 日本語 한국어'}),
            ('home.fontName', {'range': 'A12', 'name': 'Arial'}),
            ('home.fontSize', {'range': 'A12', 'size': 20}),
            ('ui.focus', {}),
            ('ui.screenshot', {'path': str(root / 'dist/release/windows-smoke.png')}),
        ]:
            stream.write((json.dumps({'id': method, 'method': method, 'params': params}) + '\n').encode())
            stream.flush()
            reply = json.loads(stream.readline())
            assert reply['ok'], reply
            if method == 'ui.inspect':
                assert reply['result']['documents'], reply
                assert reply['result']['message'] is None, reply
            print(method, 'OK')
    screenshot = root / 'dist/release/windows-smoke.png'
    assert screenshot.read_bytes().startswith(b'\x89PNG\r\n\x1a\n')
finally:
    process.terminate()
    process.wait(timeout=15)
