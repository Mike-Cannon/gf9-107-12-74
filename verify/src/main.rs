// Exhaustive weight enumerator for linear codes over GF(9).
// Written independently of the search program that found the codes.
// Element encoding: integer a+3b <-> a + b*w, w^2 = w + 1 (coefficients mod 3).
// Method: plain byte arithmetic. Every codeword = A + B + C where A, B, C are
// combinations of three disjoint groups of rows. weight(A+B+C) = n - #{p : C[p] == -(A+B)[p]}.
// Enumerates exactly one representative per projective point (leading coefficient 1).
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const W: usize = 128; // padded word length

fn fadd(x: u8, y: u8) -> u8 {
    ((x % 3 + y % 3) % 3) + 3 * ((x / 3 + y / 3) % 3)
}
fn fmul(x: u8, y: u8) -> u8 {
    // (a+bw)(c+dw) = ac + (ad+bc) w + bd w^2,  w^2 = w+1
    let (a, b, c, d) = ((x % 3) as u32, (x / 3) as u32, (y % 3) as u32, (y / 3) as u32);
    let c0 = a * c + b * d;
    let c1 = a * d + b * c + b * d;
    ((c0 % 3) + 3 * (c1 % 3)) as u8
}

fn check_field() {
    let mut ok = true;
    for x in 0..9u8 {
        for y in 0..9u8 {
            ok &= fadd(x, y) == fadd(y, x) && fmul(x, y) == fmul(y, x);
            for z in 0..9u8 {
                ok &= fadd(fadd(x, y), z) == fadd(x, fadd(y, z));
                ok &= fmul(fmul(x, y), z) == fmul(x, fmul(y, z));
                ok &= fmul(x, fadd(y, z)) == fadd(fmul(x, y), fmul(x, z));
            }
        }
    }
    for x in 0..9u8 {
        ok &= fadd(x, 0) == x && fmul(x, 1) == x && fmul(x, 0) == 0;
        ok &= (0..9u8).filter(|&y| fadd(x, y) == 0).count() == 1;
        if x != 0 {
            ok &= (1..9u8).filter(|&y| fmul(x, y) == 1).count() == 1;
        }
    }
    assert!(ok, "arithmetic is not a field");
    println!("field axioms verified exhaustively (9 elements, commutative ring with inverses)");
}

fn rank(rows: &Vec<Vec<u8>>) -> usize {
    let mut m = rows.clone();
    let n = m[0].len();
    let mut r = 0;
    for c in 0..n {
        if r == m.len() {
            break;
        }
        if let Some(p) = (r..m.len()).find(|&i| m[i][c] != 0) {
            m.swap(r, p);
            let inv = (1..9u8).find(|&y| fmul(m[r][c], y) == 1).unwrap();
            for j in 0..n {
                m[r][j] = fmul(m[r][j], inv);
            }
            for i in 0..m.len() {
                if i != r && m[i][c] != 0 {
                    let f = m[i][c];
                    let nf = fmul(f, 2); // -f, since -1 = 2
                    for j in 0..n {
                        m[i][j] = fadd(m[i][j], fmul(nf, m[r][j]));
                    }
                }
            }
            r += 1;
        }
    }
    r
}

// all 9^g combinations of the given rows; index = sum coef[i]*9^i (coef[0] = first row, least significant)
fn table(rows: &[Vec<u8>], n: usize) -> Vec<[u8; W]> {
    let g = rows.len();
    let size = 9usize.pow(g as u32);
    let mut t = vec![[0u8; W]; size];
    for idx in 0..size {
        let mut v = [0u8; W];
        let mut x = idx;
        for i in 0..g {
            let c = (x % 9) as u8;
            x /= 9;
            if c != 0 {
                for p in 0..n {
                    v[p] = fadd(v[p], fmul(c, rows[i][p]));
                }
            }
        }
        t[idx] = v;
    }
    t
}

#[inline(always)]
fn matches(a: &[u8; W], b: &[u8; W]) -> u32 {
    let mut s = 0u32;
    for chunk in 0..(W / 32) {
        let mut c = 0u8;
        for p in 0..32 {
            c += (a[chunk * 32 + p] == b[chunk * 32 + p]) as u8;
        }
        s += c as u32;
    }
    s
}

// last nonzero digit when scanning from least significant upward = most significant nonzero digit
fn leading(idx: usize, g: usize) -> Option<u8> {
    let mut lead = None;
    let mut x = idx;
    for _ in 0..g {
        let c = (x % 9) as u8;
        x /= 9;
        if c != 0 {
            lead = Some(c);
        }
    }
    lead
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let use_rows: Option<usize> = args.get(2).and_then(|s| s.parse().ok()).filter(|&x: &usize| x > 0);
    let threads: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(16);
    check_field();
    let text = std::fs::read_to_string(path).unwrap();
    let mut lines = text.lines().filter(|l| !l.trim().is_empty() && !l.starts_with("#"));
    let hdr: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (q, n, kfile) = (hdr[0], hdr[1], hdr[2]);
    assert_eq!(q, 9);
    let mut rows: Vec<Vec<u8>> = lines
        .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect();
    assert_eq!(rows.len(), kfile);
    for r in &rows {
        assert_eq!(r.len(), n);
        assert!(r.iter().all(|&x| x < 9));
    }
    if let Some(u) = use_rows {
        rows.truncate(u);
    }
    let k = rows.len();
    assert!(n <= W && k >= 9);
    let rk = rank(&rows);
    println!("file={} n={} k_used={} rank={}", path, n, k, rk);
    assert_eq!(rk, k, "rows are linearly dependent");

    // groups: G1 = rows[0..k-8], G2 = rows[k-8..k-4], G3 = rows[k-4..k]
    let g1 = k - 8;
    let t1 = Arc::new(table(&rows[0..g1], n));
    let t2 = Arc::new(table(&rows[g1..g1 + 4], n));
    let t3 = Arc::new(table(&rows[g1 + 4..k], n));
    let mut add = [[0u8; 9]; 9];
    let mut negt = [0u8; 9];
    for x in 0..9u8 {
        for y in 0..9u8 {
            add[x as usize][y as usize] = fadd(x, y);
            if fadd(x, y) == 0 {
                negt[x as usize] = y;
            }
        }
    }
    let norm3: Arc<Vec<usize>> = Arc::new((0..6561).filter(|&i| leading(i, 4) == Some(1)).collect());
    let n1 = t1.len();
    let total_outer = n1 * 6561;
    let next = Arc::new(AtomicUsize::new(0));
    let result = Arc::new(Mutex::new((vec![0u64; W + 1], usize::MAX, (0usize, 0usize, 0usize))));
    let mut hs = vec![];
    let start = std::time::Instant::now();
    for _ in 0..threads {
        let (t1, t2, t3, norm3, next, result) =
            (t1.clone(), t2.clone(), t3.clone(), norm3.clone(), next.clone(), result.clone());
        hs.push(std::thread::spawn(move || {
            let mut hist = vec![0u64; W + 1];
            let mut best = usize::MAX;
            let mut bw = (0, 0, 0);
            loop {
                let o = next.fetch_add(1, Ordering::Relaxed);
                if o >= total_outer {
                    break;
                }
                let (i1, i2) = (o / 6561, o % 6561);
                // representative per scalar class: the most significant nonzero coefficient
                // (G1 above G2 above G3) must equal 1.
                let l1 = leading(i1, g1);
                let l2 = leading(i2, 4);
                let lead = if l1.is_some() { l1 } else { l2 };
                if let Some(c) = lead {
                    if c != 1 {
                        continue;
                    }
                }
                let mut na = [255u8; W];
                for p in 0..n {
                    na[p] = negt[add[t1[i1][p] as usize][t2[i2][p] as usize] as usize];
                }
                if lead.is_some() {
                    for i3 in 0..6561 {
                        let w = n - matches(&na, &t3[i3]) as usize;
                        hist[w] += 1;
                        if w < best {
                            best = w;
                            bw = (i1, i2, i3);
                        }
                    }
                } else {
                    for &i3 in norm3.iter() {
                        let w = n - matches(&na, &t3[i3]) as usize;
                        hist[w] += 1;
                        if w < best {
                            best = w;
                            bw = (i1, i2, i3);
                        }
                    }
                }
            }
            let mut r = result.lock().unwrap();
            for i in 0..=W {
                r.0[i] += hist[i];
            }
            if best < r.1 {
                r.1 = best;
                r.2 = bw;
            }
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    let r = result.lock().unwrap();
    let total: u64 = r.0.iter().sum();
    let expect = (9u128.pow(k as u32) - 1) / 8;
    println!("projective points enumerated = {} (expected (9^k-1)/8 = {})", total, expect);
    assert_eq!(total as u128, expect);
    println!("MIN DISTANCE d = {}", r.1);
    println!("codewords of min weight (all scalar multiples counted) = {}", 8 * r.0[r.1]);
    let (i1, i2, i3) = r.2;
    let mut coef = vec![];
    let mut x = i1;
    for _ in 0..g1 {
        coef.push(x % 9);
        x /= 9;
    }
    let mut x = i2;
    for _ in 0..4 {
        coef.push(x % 9);
        x /= 9;
    }
    let mut x = i3;
    for _ in 0..4 {
        coef.push(x % 9);
        x /= 9;
    }
    println!("min-weight word coefficients (row order) = {:?}", coef);
    print!("weight distribution (weight:count incl. scalars):");
    for w in 0..=n {
        if r.0[w] > 0 {
            print!(" {}:{}", w, 8 * r.0[w]);
        }
    }
    println!();
    println!("elapsed {:.1}s", start.elapsed().as_secs_f64());
}
