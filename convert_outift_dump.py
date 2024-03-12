import json
import uuid

items = []

with open('outfits_with_guids.txt') as outfits:
    for line in outfits.readlines():
        parts = line.split(' ')
        item = ['', '', '', '']
        valid = True
        for i in range(3, len(parts)):
            if parts[i - 1] == 'P:':
                item[0] = parts[i].strip()
            elif parts[i-1] == 'M:':
                item[2] = parts[i].strip()
            elif parts[i-1] == 'S:':   
                item[1] = parts[i].strip()
            elif parts[i-1] == 'D:':
                item[3] = parts[i].strip()
        
        for i in item:
            try:
                tmp = uuid.UUID(i)
            except:
                if i != '':
                    # print('invalid ' + i)
                    valid = False
                    break
        
        if valid:
            items.append({'UnlockedLevel': 1, 'AvatarItemDesc': ','.join(item)})

items.remove(items[0])
print(json.dumps(items))
