"""Rebuild the four codes from their definition and write them to codes/.

Field: GF(9) = GF(3)[w]/(w^2 + 2w + 2), so w^2 = w + 1 (Magma's default GF(9)).
An element a + b*w is stored as the integer a + 3b:
    0 1 2 3 4   5   6  7    8
    0 1 2 w w+1 w+2 2w 2w+1 2w+2      (that is: 3=w, 4=w^2, 7=w^3, 2=w^4, 6=w^5, 8=w^6, 5=w^7)

Base code C0: the quasi-cyclic [105,11,74] code of index 3 of N. Aydin, G. Bakbouk and
J. G. G. Lambrinos (Cryptogr. Commun. 11 (2019), 891-902, Section 5.1, code 55), which is
also the entry for q=9, n=105, k=11 in M. Grassl's tables (codetables.de).  It is spanned by the 35 simultaneous cyclic shifts of
(g1 | g2 | g3), the three polynomials below, coefficients of x^0 .. x^34.

New vector j: constant on each block, (1^35 | 1^35 | (w^6)^35).

Codes: C_s = { (c + t*j | t,...,t) : c in C0, t in GF(9) } with s repeated symbols, s = 0..3.
For s = 0 this is the code C0 + <j>.  For s = 1, 2, 3 it is Construction X applied to
C0 < C0 + <j> with the [s,1,s] repetition code.

Needs only Python 3.  Usage: python construct.py
"""
G1 = [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 4, 1, 3, 1, 4, 5, 8, 3, 5, 4, 1, 2, 7, 0, 6, 5, 7, 8, 4, 1, 6, 0, 6]
G2 = [0, 2, 0, 5, 4, 1, 0, 5, 5, 6, 2, 3, 0, 3, 4, 0, 3, 7, 3, 1, 7, 5, 5, 3, 7, 6, 0, 2, 3, 3, 2, 7, 2, 4, 1]
G3 = [3, 3, 8, 3, 1, 4, 3, 1, 1, 4, 3, 7, 8, 6, 6, 3, 0, 2, 5, 2, 2, 1, 6, 4, 4, 3, 1, 6, 2, 1, 5, 6, 3, 8, 4]
J_BLOCKS = (1, 1, 8)
M = 35


def add(x, y):
    return (x % 3 + y % 3) % 3 + 3 * ((x // 3 + y // 3) % 3)


def mul(x, y):
    a, b, c, d = x % 3, x // 3, y % 3, y // 3
    return (a * c + b * d) % 3 + 3 * ((a * d + b * c + b * d) % 3)


def neg(x):
    return (-(x % 3)) % 3 + 3 * ((-(x // 3)) % 3)


INV = {x: y for x in range(1, 9) for y in range(1, 9) if mul(x, y) == 1}


def row_reduce(rows):
    """Reduced row echelon basis of the span of rows."""
    rows = [r[:] for r in rows]
    basis, n = [], len(rows[0])
    for col in range(n):
        piv = next((r for r in rows if r[col]), None)
        if piv is None:
            continue
        rows.remove(piv)
        piv = [mul(INV[piv[col]], x) for x in piv]
        for group in (rows, basis):
            for r in group:
                if r[col]:
                    f = neg(r[col])
                    for i in range(n):
                        if piv[i]:
                            r[i] = add(r[i], mul(f, piv[i]))
        basis.append(piv)
        rows = [r for r in rows if any(r)]
    return sorted(basis, reverse=True)


def base_code():
    blocks = [G1[:], G2[:], G3[:]]
    rows = []
    for _ in range(M):
        rows.append(blocks[0] + blocks[1] + blocks[2])
        blocks = [[b[-1]] + b[:-1] for b in blocks]      # multiply by x modulo x^35 - 1
    basis = row_reduce(rows)
    assert len(basis) == 11, 'the base code should have dimension 11'
    return basis


def code(s):
    j = [v for v in J_BLOCKS for _ in range(M)]
    return [r + [0] * s for r in base_code()] + [j + [1] * s]


def write(path, rows, title):
    with open(path, 'w') as f:
        f.write('# %s' % title + chr(10) + '9 %d %d' % (len(rows[0]), len(rows)) + chr(10))
        for r in rows:
            f.write(' '.join(map(str, r)) + chr(10))


if __name__ == '__main__':
    for s, d in ((0, 72), (1, 73), (2, 74), (3, 74)):
        rows = code(s)
        assert len(row_reduce(rows)) == 12, 'the code should have dimension 12'
        name = 'codes/gf9_%d_12_%d.txt' % (105 + s, d)
        write(name, rows, '[%d,12,%d] code over GF(9); integer a+3b means a+b*w, w^2=w+1' % (105 + s, d))
        print('wrote', name)
