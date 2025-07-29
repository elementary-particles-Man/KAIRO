import json
from datetime import datetime
from pathlib import Path

reg_path = Path('agent_registry.json')
list_path = Path('agent_state/AGENT_LIST.md')
list_path.parent.mkdir(exist_ok=True)

if not reg_path.exists():
    print('No registry found')
    exit(1)

with reg_path.open() as f:
    entries = json.load(f)

with list_path.open('w') as f:
    f.write('# Registered Agents\n\n')
    f.write('| Name | P Address | Last Contact | Deleted |\n')
    f.write('|-----|-----------|-------------|---------|\n')
    for e in entries:
        last = e.get('last_contact') or 'N/A'
        if isinstance(last, str) and last.endswith('Z'):
            # ensure human readable
            try:
                last = datetime.fromisoformat(last.replace('Z','+00:00')).strftime('%Y-%m-%d %H:%M')
            except Exception:
                pass
        f.write(f"| {e.get('name','')} | {e.get('p_address','')} | {last} | {e.get('deleted', False)} |\n")
print(f'Updated {list_path}')
