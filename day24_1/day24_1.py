import re
import queue
import sys

def parse_input(fh):
    """Return a dict of wires[values] and a list of tuples of (wire_in, op, wire_in, wire_out)."""

    with open(fh, 'r') as f:
        lines = f.readlines()

    p1 = r"([a-z0-9]{3}): ([01])"
    p2 = r"([a-z0-9]{3}) ([ANDORX]{2,3}) ([a-z0-9]{3}) -> ([a-z0-9]{3})"

    values = {}
    conns = []

    for line in lines:
        m1 = re.search(p1, line)
        if m1 is not None:
            values[m1.groups()[0]] = int(m1.groups()[1])
            continue
        m2 = re.search(p2, line)
        if m2 is not None:
            conns.append((m2.groups()))

    return (values, conns)

def get_new_wire_value(wires, conn):

    in1, op, in2, out = conn

    if wires.get(in1) is None or wires.get(in2) is None:
        return

    if op == 'OR':
        return int(wires.get(in1) or wires.get(in2))
    elif op == 'AND':
        return int(wires.get(in1) and wires.get(in2))
    elif op == 'XOR':
        return int(wires.get(in1) + wires.get(in2) == 1)




if __name__ == '__main__':

    wires, conns = parse_input(sys.argv[1])

    q = queue.Queue()
    for c in conns:
        q.put(c)

    while not q.empty():
        c = q.get()
        new_wire = get_new_wire_value(wires, c)
        if new_wire is None:
            q.put(c)
        else:
            wires[c[3]]=new_wire

    zkeys = sorted([k for k in wires.keys() if k[0]=='z'], reverse=True)
    binum = '0b'+''.join([str(wires[k]) for k in zkeys])
    num = int(binum, 2)

    print('answer:', num)
