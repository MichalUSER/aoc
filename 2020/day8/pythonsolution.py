with open("instructions.txt") as f:
    d = f.readlines()

acc = 0
inst = 0
executed = set()
while inst not in executed:
    i = d[inst].split()
    executed.add(inst)
    c = i[0]
    v = int(i[1])
    if c == 'nop':
        inst += 1
    elif c == 'acc':
        acc += v
        inst += 1
    elif c == 'jmp':
        inst += v

print(acc)
non_looping = set()
looping = set()
for sinst in range(0, len(d)):
    inst = sinst
    executed = set()
    while inst not in executed:
        if inst in non_looping or inst >= len(d):
            non_looping |= executed
            break
        if inst in looping:
            looping |= executed
        i = d[inst].split()
        executed.add(inst)
        c = i[0]
        v = int(i[1])
        if c == 'nop':
            inst += 1
        elif c == 'acc':
            acc += v
            inst += 1
        elif c == 'jmp':
            inst += v
    looping |= executed

ch = False
acc = 0
inst = 0
while inst < len(d):
    i = d[inst].split()
    c = i[0]
    v = int(i[1])
    if c == 'nop':
        if inst + v in non_looping and not ch:
            ch = True
            inst += v
        else:
            inst += 1
    elif c == 'acc':
        acc += v
        inst += 1
    elif c == 'jmp':
        if inst + 1 in non_looping and not ch:
            ch = True
            inst += 1
        else:
            inst += v

print(acc)
