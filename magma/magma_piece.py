"""Magma input that checks part of a code: the codewords whose last s message
coordinates are zero or proportional to the lo-th .. hi-th point of the projective
space PG(s-1, 9).  Usage: python magma_piece.py <name> <s> <lo> <hi>   (reads ../codes/<name>.txt)"""
import sys


def read_matrix(path):
    lines = [l for l in open(path) if l.strip() and not l.startswith('#')]
    return [list(map(int, l.split())) for l in lines[1:]]


name, s, lo, hi = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
rows = read_matrix(f'../codes/{name}.txt')
k, n = len(rows), len(rows[0])
flat = ','.join(str(x) for r in rows for x in r)
print(f'''F<w> := GF(9);
assert w^2 eq w + 1;
elts := [F | 0, 1, 2, w, w+1, w+2, 2*w, 2*w+1, 2*w+2];
L := [{flat}];
M := Matrix(F, {k}, {n}, [elts[x+1] : x in L]);
assert Rank(M) eq {k};
s := {s};
D := RowSubmatrix(M, 1, {k} - s);
T := RowSubmatrix(M, {k} - s + 1, s);
// projective points of GF(9)^s: first nonzero coordinate equal to 1
V := VectorSpace(F, s);
pts := [v : v in V | v ne 0 and v[Min([i : i in [1..s] | v[i] ne 0])] eq 1];
assert #pts eq (9^s - 1) div 8;
t0 := Cputime();
best := 1000;
for idx in [{lo}..{hi}] do
    v := pts[idx];
    C := LinearCode(VerticalJoin(D, Matrix(F, 1, {n}, Eltseq(v * T))));
    assert Dimension(C) eq {k} - s + 1;
    d := MinimumDistance(C);
    best := Min(best, d);
    printf "piece %o d %o time %o\n", idx, d, Cputime(t0);
end for;
printf "RESULT {name} s={s} pieces {lo}..{hi} of %o min %o\n", #pts, best;
''')
