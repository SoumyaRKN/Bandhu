import re
import sys

with open('ROADMAP.md', 'r') as f:
    content = f.read()

status_map = {
    1: 'Completed', 2: 'Completed', 3: 'Completed',
    4: 'Completed', 5: 'Completed', 6: 'Completed', 7: 'Completed',
    8: 'Completed', 9: 'Completed', 10: 'Pending',
    11: 'Completed', 12: 'Completed', 13: 'Completed', 14: 'Completed', 15: 'Pending',
    16: 'Completed', 17: 'Completed', 18: 'Completed', 19: 'Completed',
    20: 'Completed', 21: 'Completed', 22: 'Completed', 23: 'Pending', 24: 'Completed',
    25: 'Completed', 26: 'Completed', 27: 'Completed', 28: 'Completed',
    29: 'Completed', 30: 'Completed', 31: 'Pending', 32: 'Pending', 33: 'Pending',
    34: 'Completed', 35: 'Completed', 36: 'Pending', 37: 'Completed',
    38: 'Pending', 39: 'Completed', 40: 'Pending', 41: 'Pending',
    42: 'Completed', 43: 'Pending', 44: 'Completed', 45: 'Pending',
    46: 'Pending', 47: 'Pending', 48: 'Pending',
    49: 'Pending', 50: 'Pending', 51: 'Pending', 52: 'Pending',
    53: 'Pending', 54: 'Pending', 55: 'Pending', 56: 'Pending',
    57: 'Pending', 58: 'Pending', 59: 'Pending', 60: 'Pending',
    61: 'Pending', 62: 'Pending', 63: 'Pending', 64: 'Pending',
    65: 'Pending', 66: 'Pending', 67: 'Pending', 68: 'Pending',
    69: 'Pending', 70: 'Pending', 71: 'Pending', 72: 'Pending',
    73: 'Pending', 74: 'Pending', 75: 'Pending', 76: 'Pending',
    77: 'Pending', 78: 'Pending', 79: 'Pending', 80: 'Pending',
    81: 'Pending', 82: 'Pending', 83: 'Pending', 84: 'Pending',
    85: 'Pending', 86: 'Pending', 87: 'Pending', 88: 'Pending',
    89: 'Pending', 90: 'Pending', 91: 'Pending', 92: 'Pending',
    93: 'Pending', 94: 'Pending', 95: 'Pending', 96: 'Pending',
    97: 'Pending', 98: 'Pending', 99: 'Pending', 100: 'Pending',
    101: 'Pending', 102: 'Pending', 103: 'Pending', 104: 'Pending',
    105: 'Pending', 106: 'Pending', 107: 'Pending', 108: 'Pending',
    109: 'Pending', 110: 'Pending', 111: 'Pending', 112: 'Pending',
    113: 'Pending', 114: 'Pending', 115: 'Pending', 116: 'Pending',
    117: 'Pending', 118: 'Pending', 119: 'Pending', 120: 'Pending',
    121: 'Pending', 122: 'Pending', 123: 'Pending', 124: 'Pending',
    125: 'Pending', 126: 'Pending', 127: 'Pending', 128: 'Pending',
    129: 'Pending', 130: 'Pending', 131: 'Pending', 132: 'Pending',
    133: 'Pending', 134: 'Pending', 135: 'Pending',
}

lines = content.split('\n')
new_lines = []
for line in lines:
    match = re.match(r'^\|\s*(\d+)\s*\|.*\|\s*`([^`]+)`\s*\|$', line)
    if match:
        task_num = int(match.group(1))
        if task_num in status_map:
            line = re.sub(r'`[^`]+`', '\`{}\`'.format(status_map[task_num]), line)
    new_lines.append(line)

content = '\n'.join(new_lines)

content = content.replace('| `pending` | Not yet started |', '| `Pending` | Not yet started |')
content = content.replace('| `in_progress` | Currently being worked on |', '| `Pending` | Currently being worked on |')
content = content.replace('| `completed` | Finished and verified |', '| `Completed` | Finished and verified |')
content = content.replace('| `blocked` | Waiting on dependency or external factor |', '| `Pending` | Waiting on dependency or external factor |')

with open('ROADMAP.md', 'w') as f:
    f.write(content)

print('ROADMAP.md updated successfully')
