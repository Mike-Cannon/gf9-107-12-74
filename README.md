# A linear [107,12,74] code over GF(9), and three derived codes

**Author:** Mike Cannon (independent researcher, https://github.com/Mike-Cannon)
**Date:** 27 November 2026
**DOI:** [10.5281/zenodo.24611880](https://doi.org/10.5281/zenodo.24611880)
**Repository:** https://github.com/Mike-Cannon/gf9-107-12-74

## Result

There is a linear code over GF(9) with parameters **[107,12,74]**. Codes with parameters
[105,12,72], [106,12,73] and [108,12,74] follow from the same construction.

Each has a larger minimum distance than the lower bound shown in M. Grassl's tables of best
known linear codes [1] on 27 November 2026. Magma's built-in copy of the tables (V2.29-10)
gives the same four lower bounds.

| Code | Lower bound in [1] | Upper bound in [1] | Words of minimum weight |
|---|---|---|---|
| [105,12,72] | 71 | 83 | 3 080 |
| [106,12,73] | 71 | 84 | 3 080 |
| [107,12,74] | 72 | 85 | 8 680 |
| [108,12,74] | 73 | 86 | 5 600 |

This is one construction and one new observation. The [107,12,74] code is the main result:
deleting its last coordinate, or its last two, gives the [106,12,73] and [105,12,72] codes,
and adding a zero coordinate gives a [108,12,74] code. The [108,12,74] code in this deposit is
the one obtained by continuing the construction with a third repeated symbol; it has fewer
words of minimum weight. None of the codes is known to be optimal.

The claim is a comparison with the bounds displayed in [1] on that date. It is not a claim
that nobody has found such a code; see "Earlier work" below.

## Construction

Let GF(9) = GF(3)[w]/(w^2 + 2w + 2), so that w^2 = w + 1 and w is primitive.

1. **Base code.** C0 is the quasi-cyclic [105,11,74] code of index 3, with circulants of size
   m = 35, of Aydin, Bakbouk and Lambrinos: code 55 in Section 5.1 of [3]. It is also the
   entry for q = 9, n = 105, k = 11 in [1], and it is in Chen's database of quasi-twisted
   codes [2]. It is spanned by the 35 simultaneous cyclic shifts of three
   polynomials g1, g2, g3, which are written out in `construct.py`. With h, f2 and f3 as
   printed in [3], coefficients in ascending order, h*g1 = 0, g2 = f2*g1 and g3 = f3*g1
   modulo x^35 - 1. It has 5 600 words of weight 74.
2. **One more dimension.** Let j = (1^35 | 1^35 | (w^6)^35), constant on each block. Exhaustive
   enumeration shows that the smallest weight of a word of C0 + <j> outside C0 is 72, with
   3 080 such words. So C0 + <j> is a [105,12,72] code. It is again quasi-cyclic of index 3.
3. **Construction X with a repetition code.** For s = 1, 2, 3 the code

       C_s = { (c + t*j | t, ..., t) : c in C0, t in GF(9) }      (s copies of t)

   has parameters [105 + s, 12, min(74, 72 + s)], that is [106,12,73], [107,12,74] and
   [108,12,74].

The only new ingredient is the choice of j and the fact that its coset has minimum weight 72.
Everything else is known:

- Step 3 is Construction X [4] applied to the pair C0 < C0 + <j>, with the [s,1,s] repetition
  code as auxiliary code. Since both codes are quasi-cyclic, it is Construction X on a chain
  of quasi-cyclic codes, as in Grassl and White [9].
- The distance formula for adding one vector and appending a repetition code is Lemma 3 of
  Cochinescu [5]. That paper applies it with block-constant vectors to quasi-cyclic codes of
  dimension up to 8, and the idea of trying it on the codes in [2] came from there.
- Adding block-constant vectors to quasi-cyclic codes goes back further; see for example
  Aydin, Connolly and Murphree [10] for binary codes.

In our scan of the 91 block-constant vectors (up to scalar multiples), this was the only one
that gave 72. The scan itself is not part of this deposit.

## Earlier work on these parameters

- The four lower-bound constructions in [1] for n = 105, ..., 108 and k = 12 are dated
  2003-10-28, 2003-10-21, 2003-10-21 and 2004-09-10.
- A [106,12,72] code over GF(9), obtained by Construction XX, was published on 7 November
  2026 in the repository [6]. The [106,12,73] code here improves on it by one.
- We found no code over GF(9) of these lengths and dimension reaching these distances in [1],
  [2], [5], [6], [7], or in a search of recent papers and public repositories.
- In [3], the codes over GF(9) of dimension 12 listed in Section 5 have lengths 112, 120,
  123 and 130, and the one of dimension 13 has length 123. None has length 105 to 108, and
  puncturing or shortening them does not guarantee the distances above. Section 5.2 of [3] also
  mentions 82 further codes obtained by extending, shortening or puncturing, which were sent
  to [1] and are not listed individually.
- The search was not exhaustive, and [1] does not yet show some codes that others have
  reported in recent months. A code with these parameters may exist elsewhere.

## Files

| File | Contents |
|---|---|
| `codes/gf9_107_12_74.txt` (and 105, 106, 108) | Generator matrices |
| `construct.py` | Rebuilds the four matrices from the three polynomials and j. Needs only Python 3. |
| `verify/` | A program that visits every codeword (one per scalar multiple) and reports the weight distribution |
| `verification_output.txt` | Its output for the four codes |
| `magma/` | Input for Magma [8], and the record of a piecewise check in the Magma calculator |

**Matrix format.** A comment line, then `9 n k`, then k rows of n integers. The integer a + 3b
stands for the field element a + b*w:

    0 1 2 3 4   5   6  7    8
    0 1 2 w w+1 w+2 2w 2w+1 2w+2

Rows 1 to 11 generate C0 (followed by zeros); row 12 is j followed by ones.

## Verification

**What a reader can check from this deposit.**

1. `construct.py` rebuilds the four matrices from the polynomials of C0 and the vector j.
2. The program in `verify/` visits all (9^12 - 1)/8 = 35 303 692 060 codewords up to scalar
   multiples and counts weights. Its output, in `verification_output.txt`, gives minimum
   distances 72, 73, 74 and 74, with 3 080, 3 080, 8 680 and 5 600 words of minimum weight,
   and the full weight distributions (the zero word is not counted).

       cd verify
       cargo build --release
       target/release/gf9_enum ../codes/gf9_107_12_74.txt 0 16

   The last argument is the number of threads. A run took about 20 seconds per code with 16
   threads on the machine used. The line "min-weight word coefficients" shows one example
   word and can differ from run to run.
3. **Magma.** The [107,12,74] code was checked in the Magma online calculator (V2.29-10) by
   splitting it into 820 subcodes of dimension 9 that together contain every codeword.
   `MinimumDistance` returned 74 for each of them. The argument, the scripts and the saved
   answers are in `magma/`; only the parsed answers were kept, not the calculator's raw
   replies. The four `gf9_*.m` files, which call `MinimumDistance` on the whole codes, have
   not been run to completion: the calculator stops after 60 seconds and the author has no
   Magma licence.

   The Magma check covers the [107,12,74] code. Puncturing it gives the lower bounds 73 and
   72 at lengths 106 and 105. The [108,12,74] code was checked by enumeration only.

**What the author reports but this deposit does not show.** Three other programs gave the
same four minimum distances and the same numbers of words of minimum weight: the search
program that found the codes (bit-sliced arithmetic, on CPU and on GPU); a program that
enumerates C0 and derives the weights of each coset from block statistics; and a program
written by a different AI system that was given only the matrices. The program in `verify/`
and the last two were written by AI systems asked to try to refute the claim.

## How the codes were found, and use of AI

The search, construction and verification programs were written and run by AI coding agents
under the author's direction, on a mix of owned and rented hardware. AI systems from two
different developers were used as independent reviewers, of the result and then of this
text. The codes came from a scan that applied the construction above to the GF(9) codes in
[2] that meet the lower bounds of [1].

The parameters and weight distributions stated above do not rest on trust in these tools:
they can be confirmed with the files in this deposit.

## References

1. M. Grassl, *Bounds on the minimum distance of linear codes and quantum codes*,
   http://www.codetables.de. Accessed 27 November 2026.
2. E. Z. Chen, *Online database of quasi-twisted codes*, http://databases.cs.hkr.se/qtcodes/.
   Accessed 27 November 2026.
3. N. Aydin, G. Bakbouk and J. G. G. Lambrinos, New linear codes over non-prime fields,
   *Cryptogr. Commun.* 11 (2019), 891-902. doi:10.1007/s12095-018-0333-2
4. N. J. A. Sloane, S. M. Reddy and C.-L. Chen, New binary codes, *IEEE Trans. Inform. Theory*
   18 (1972), 503-510.
5. S. Cochinescu, New linear codes over GF(7), GF(8) and GF(9) from quasi-cyclic codes and
   Construction X, arXiv:2611.14433 (2026).
6. R. Pinho, *math-certificates*, https://github.com/rpinho/math-certificates, folder
   `codetables/q9_n106_k12_d72`. Accessed 27 November 2026.
7. C. Yu and S. Zhu, Construction of new linear codes with good parameters from group rings
   and skew group rings, *Discrete Math.* 348 (2025), 114349, and the file
   https://github.com/Yucong1234/Generator_matrix_of_new_linear_codes.
8. W. Bosma, J. Cannon and C. Playoust, The Magma algebra system I: The user language,
   *J. Symbolic Comput.* 24 (1997), 235-265.
9. M. Grassl and G. White, New codes from chains of quasi-cyclic codes, *Proc. IEEE Int.
   Symp. Inform. Theory (ISIT 2005)*, 2095-2099. doi:10.1109/ISIT.2005.1523715
10. N. Aydin, N. Connolly and J. Murphree, New binary linear codes from quasi-cyclic codes
    and an augmentation algorithm, *Appl. Algebra Engrg. Comm. Comput.* 28 (2017), 339-350.
    doi:10.1007/s00200-017-0327-x

## Licence

The generator matrices (including the copies inside the Magma files) and this text:
CC BY 4.0. The programs: MIT licence. See `LICENSE.txt`.
