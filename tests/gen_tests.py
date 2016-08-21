import operator
import random

NUM_TESTS = 1000

def write_test(op, x, y, f):
    def h(x): return hex(x)[2:]

    ans = op(x, y)
    f.write('{}\t{}\t{}\n'.format(h(x), h(y), h(ans)))

def gen_sub():
    with open('subtraction.data', 'w') as f:
        for _ in range(NUM_TESTS):
            x = random.randint(2 ** 255, 2 ** 511)
            y = random.randint(2 ** 255, 2 ** 511)
            (x, y) = (max(x, y), min(x, y))
            write_test(operator.sub, x, y, f)

def gen_add():
    with open('addition.data', 'w') as f:
        for _ in range(NUM_TESTS):
            # Generate two big numbers and write their sums
            x = random.randint(2 ** 255, 2 ** 511)
            y = random.randint(2 ** 255, 2 ** 511)
            write_test(operator.add, x, y, f)

def gen_mul():
    with open('multiplication.data', 'w') as f:
        for _ in range(NUM_TESTS):
            # Generate two big numbers and write their sums
            x = random.randint(2 ** 100, 2 ** 254)
            y = random.randint(2 ** 100, 2 ** 254)
            write_test(operator.mul, x, y, f)

def gen_rem():
    with open('remainder.data', 'w') as f:
        for _ in range(NUM_TESTS):
            # Generate two big numbers and write their sums
            x = random.randint(2 ** 255, 2 ** 511)
            y = random.randint(2 ** 20, 2 ** 511)
            write_test(operator.mod, x, y, f)

def gen_div():
    with open('division.data', 'w') as f:
        for _ in range(NUM_TESTS):
            # Generate two big numbers and write their sums
            x = random.randint(2 ** 255, 2 ** 511)
            y = random.randint(2 ** 20, 2 ** 511)
            write_test(operator.floordiv, x, y, f)



if __name__ == '__main__':
    gen_div()
    gen_rem()
    gen_mul()
    gen_add()
    gen_sub()
