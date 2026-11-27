"""Check a code piece by piece in the Magma online calculator (60 s limit per request).

The code C is split by its last s message coordinates into (9^s-1)/8 subcodes
<D, vT>, whose minimum distances together give d(C).  Answers are saved after
every request, so a run can be stopped and resumed.

Usage (from this folder):
    python magma_run.py <name> <s> [batch] [first] [last] [tag]

first, last   only do pieces first..last (default: all)
tag           suffix for the file of saved answers, so that two ranges can run at once

Answers go to pieces_<name>_s<s><tag>.json.  Pieces already in that file are skipped, so
with the published pieces_gf9_107_12_74_s4.json in place the script reports FINISHED at
once.  For a fresh run, move that file away or give a tag.

Only the parsed line "piece <i> d <d>" of each reply is kept, not the raw reply.
Please be considerate of the free calculator: this makes about one request a minute.
"""
import html, json, os, re, subprocess, sys, time, urllib.parse, urllib.request

name, s = sys.argv[1], int(sys.argv[2])
batch = int(sys.argv[3]) if len(sys.argv) > 3 else 8
points = (9 ** s - 1) // 8
first = int(sys.argv[4]) if len(sys.argv) > 4 else 1
last = int(sys.argv[5]) if len(sys.argv) > 5 else points
tag = sys.argv[6] if len(sys.argv) > 6 else ''
state_path = f'pieces_{name}_s{s}{tag}.json'
state = json.load(open(state_path)) if os.path.exists(state_path) else {}
URL = 'http://magma.maths.usyd.edu.au/xml/calculator.xml'


def ask(lo, hi):
    src = subprocess.run([sys.executable, 'magma_piece.py', name, str(s), str(lo), str(hi)],
                         capture_output=True, text=True).stdout
    data = urllib.parse.urlencode({'input': src}).encode()
    try:
        with urllib.request.urlopen(URL, data, timeout=120) as r:
            text = html.unescape(re.sub(r'<[^>]+>', '\n', r.read().decode('utf-8', 'replace')))
    except Exception as e:
        return {}
    got = {int(i): int(d) for i, d in re.findall(r'piece (\d+) d (\d+) time', text)}
    return {i: d for i, d in got.items() if lo <= i <= hi}


idx = first
while idx <= last:
    if str(idx) in state:
        idx += 1
        continue
    hi = idx
    while hi < last and hi - idx + 1 < batch and str(hi + 1) not in state:
        hi += 1
    got = ask(idx, hi)
    for i, d in got.items():
        state[str(i)] = d
    json.dump(state, open(state_path, 'w'))
    print(time.strftime('%H:%M:%S UTC', time.gmtime()), f'asked {idx}..{hi}, got {len(got)}; done {len(state)}/{last}; '
          f'min so far {min(state.values()) if state else None}', flush=True)
    if not got:
        batch = max(1, batch // 2)       # too slow or refused: ask for less next time
        time.sleep(20)
    time.sleep(3)
print('FINISHED', name, 'pieces', len(state), 'of', last, 'minimum distance', min(state.values()), flush=True)
