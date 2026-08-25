import json, sys
data = json.load(open('benchmarks/retrieval/ground-truth.json'))
errors = []
for q in data:
    if q.get('verification_status') != 'verified':
        continue
    if not q.get('expected_entities'):
        errors.append(f"{q['id']} has no expected_entities")
    if not q.get('repo'):
        errors.append(f"{q['id']} missing repo")
print('Errors:', errors)
print('Verified count:', sum(1 for q in data if q.get('verification_status')=='verified'))
