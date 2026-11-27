Magma checks
============

gf9_*.m
    Each file defines one of the four codes and calls MinimumDistance. Use these in a licensed
    copy of Magma. They have NOT been run to completion by the author: in the free online
    calculator the computation does not finish within its 60-second limit.

Piecewise check of the [107,12,74] code in the Magma online calculator
    The code C is split into 820 subcodes. Write a message as (u, v) with u its first 8
    coordinates and v its last 4. Let D be spanned by the first 8 rows of the generator
    matrix and let T be its last 4 rows. For each of the (9^4 - 1)/8 = 820 points v of the
    projective space PG(3,9), the subcode <D, vT> has dimension 9. A nonzero codeword with
    v = 0 lies in D, which is inside every subcode; one with v nonzero lies in the subcode of
    the point proportional to v. So d(C) is the smallest of the 820 minimum distances.

    magma_piece.py   writes the Magma input for a range of pieces; it reads the matrix
                     ../codes/gf9_107_12_74.txt
    magma_run.py     sends the pieces to the calculator, a few at a time, and saves the answers
    pieces_gf9_107_12_74_s4.json   the minimum distance Magma returned for each piece
    run_log_*.txt    the progress logs (times are UTC)

RESULT: complete, 27 November 2026, Magma V2.29-10 (the version the calculator reported).
The saved answers contain 74 for every piece 1 to 820. Hence d(C) = 74.

How the run was actually done, and what was kept
    It was done in three stretches: one job for pieces 1-192, then two jobs in parallel for
    pieces 193-506 and 507-820. Each job counted its progress in its own file, which is why
    the "done x/y" figures differ between the three logs; the three answer files were merged
    into the one published here. Only the parsed minimum distance of each piece was kept, not
    the calculator's raw replies. The scripts here are the ones used, tidied afterwards: they
    now read the matrix from ../codes and the range options are documented. The Magma
    statements sent to the calculator are unchanged.

    That every piece gives exactly 74 is expected: D already contains a word of weight 74,
    and D lies in every piece. The content of the check is that no piece goes below 74.
