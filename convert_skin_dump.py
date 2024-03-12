import json
import uuid

result = []

with open('skins.txt') as skins:
    for line in skins.readlines():
        line = line.replace('\n', '')
        parts = line.split(' ')
        if len(parts) != 2:
            continue
        try:
            tmp = uuid.UUID(parts[1])
        except:
            continue

        result.append({
            'PrefabName': parts[0],
            'ModificationGuid': parts[1],
            'UnlockedLevel': 1,
            'Favorited': False,
        })

print(json.dumps(result))
