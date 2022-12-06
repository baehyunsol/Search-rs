def next(n):

    if n % 2 == 0:
        return n // 2

    else:
        return n * 3 + 1

def gen(n):

    result = []

    while n > 1:
        result.append(str(n))
        n = next(n)

    result.append(str(n))
    return " -> ".join(result)

for i in range(600, 1000):

    with open(str(i) + ".txt", 'w') as f:
        f.write(gen(i))