import json
import os
import collections
c = collections.Counter()
for file in os.listdir('.'):
    try:
        data = json.load(open(file))
        c.update(data)
    except: continue

for k,v in c.items():
    print(k, v, sep='\t')