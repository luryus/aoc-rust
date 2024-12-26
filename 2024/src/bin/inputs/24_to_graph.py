with open('24.txt', 'r') as f:
    x = f.readlines()
    x = x[x.index('\n')+1:]

gates = { }
edges = []

for i, l in enumerate(x):
    s1, g, s2, _arr, res = l.strip().split(' ')
    gates[f"{g}{i}"] =  f'    {g}{i} [shape=triangle, label="{g}"]'
    if s1 not in gates:
        gates[s1] = f'    {s1} [shape=circle, label="{s1}"]'
    if s2 not in gates:
        gates[s2] =  f'    {s2} [shape=circle, label="{s2}"]'
    if res not in gates:
        gates[res] =  f'    {res} [shape=circle, label="{res}"]'

    edges.append(f'   {s1} -- {g}{i}')
    edges.append(f'   {s2} -- {g}{i}')
    edges.append(f'   {g}{i} -- {res}')

print('graph {')
for l in gates.values():
    print(l)
print()
for e in edges:
    print(e)
print('}')