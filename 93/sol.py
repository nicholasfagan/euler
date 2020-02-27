


import itertools as itt



def doAllBracketedOps(a,b,c,d,o1,o2,o3):
    a=str(a)
    b=str(b)
    c=str(c)
    d=str(d)
    o1=str(o1)
    o2=str(o2)
    o3=str(o3)
    # (a + b + c + d)
    yield (a + o1 + b + o2 + c + o3 + d)
    # (a + b) + (c + d)
    yield ("(" + a + o1 + b + ")" + o2 + "(" + c + o3 + d + ")")
    # (a + b + c) + d
    yield ("(" + a + o1 + b + o2 + c + ")" + o3 + d)
    # a + (b + c + d)
    yield (a + o1 + "(" + b + o2 + c + o3 + d + ")")
    # ( ( a + b ) + c ) + d
    yield ("(" + "(" + a + o1 + b + ")" + o2 + c + ")" + o3 + d)
    # ( a + (b + c) ) + d
    yield ("(" + a + o1 + "(" + b + o2 + c + ")" + o3 + d + ")")
    # a + ((b + c) + d)
    yield (a + o1 + "(" + "(" + b + o2 + c + ")" + o3 + d + ")")
    # a + (b + (c + d))
    yield (a + o1 + "(" + b + o2 + "(" + c + o3 + d + ")" +")")
ops = ["+","-","*","/"]

def gen_solved_eq(lst):
    for i1 in range(4):
        o1 = ops[i1]
        for i2 in range(4):
            o2 = ops[i2]
            for i3 in range(4):
                o3 = ops[i3]
                for l in itt.permutations(lst):
                    for n in doAllBracketedOps(l[0],l[1],l[2],l[3],o1,o2,o3):
                        try:
                            evaln = eval(n)
                            if int(evaln) == evaln and evaln > 0:
                                yield int(evaln)
                        except ZeroDivisionError:
                            continue

maxrun = 0
maxtuple = (0,0,0,0)
for a,b,c,d in itt.combinations(range(1,10), 4):
    nums = list(gen_solved_eq([a,b,c,d]))
    nums.sort()
    filtered = []
    prev = 0
    for n in nums:
        if n != prev:
            prev = n
            filtered.append(n)
    prev = 0
    run = 0
    for n in filtered:
        if n == prev + 1:
            run += 1
        else:
            if run > maxrun:
                maxrun = run
                maxtuple = (a,b,c,d)
            run = 1
        prev = n
a,b,c,d = maxtuple
print(str(a)+str(b)+str(c)+str(d) + ", for a run of " + str(maxrun))
