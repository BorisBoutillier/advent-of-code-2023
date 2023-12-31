use aoc_commons::*;
use itertools::Itertools;
use num_bigint::BigInt;
type I = BigInt;

#[derive(Clone, Debug)]
struct Vec3 {
    x: I,
    y: I,
    z: I,
}
impl Vec3 {
    fn new(d: &[I]) -> Self {
        Vec3 {
            x: d[0].clone(),
            y: d[1].clone(),
            z: d[2].clone(),
        }
    }
}

#[derive(Clone, Debug)]
struct Hailstone {
    p: Vec<I>,
    v: Vec<I>,
}
impl Hailstone {
    fn parse(line: &str) -> Hailstone {
        let (p, v) = line.split_once(" @ ").unwrap();
        let p = p
            .split(',')
            .map(|d| d.trim().parse::<I>().unwrap())
            .collect();
        let v = v
            .split(',')
            .map(|d| d.trim().parse::<I>().unwrap())
            .collect();
        Hailstone { p, v }
    }
}
const MIN_V: i64 = 200_000_000_000_000;
const MAX_V: i64 = 400_000_000_000_000;
pub fn solver(part: Part, input: &str) -> String {
    match part {
        Part::Part1 => part1(input, MIN_V.into(), MAX_V.into()),
        Part::Part2 => part2(input),
    }
    .to_string()
}
// p1.x+t1*v1.x = p2.x+t2*v2.x;
// p1.y+t1*v1.y = p2.y+t2*v2.y;
// t1*v1.x = (p2.x-p1.x)+t2*v2.x;
// t1*v1.y = (p2.y-p1.y)+t2*v2.y;
// 0 = (p2.x-p1.x)*v1.y-(p2.y-p1.y)*v1.x+t2(v2.x*v1.y-v2.y*v2.x)
// t2 = -((p2x-p1x)*v1y-(p2y-p1y)*v1x)/(v2x*v1y-v2y*v1x)
fn part1(input: &str, min_v: I, max_v: I) -> I {
    let hailstones = input.lines().map(Hailstone::parse).collect::<Vec<_>>();
    let mut count = 0;
    for h in hailstones.iter().combinations(2) {
        //println!("H0: {:?}", h[0]);
        //println!("H1: {:?}", h[1]);
        let dv = &h[1].v[0] * &h[0].v[1] - &h[1].v[1] * &h[0].v[0];
        if dv != 0.into() {
            let t2 = -((&h[1].p[0] - &h[0].p[0]) * &h[0].v[1]
                - (&h[1].p[1] - &h[0].p[1]) * &h[0].v[0])
                / dv;
            let t1 = (&t2 * &h[1].v[0] + &h[1].p[0] - &h[0].p[0]) / &h[0].v[0];
            let x = &t1 * &h[0].v[0] + &h[0].p[0];
            let y = &t1 * &h[0].v[1] + &h[0].p[1];
            if t2 > 0.into()
                && t1 > 0.into()
                && (&min_v..=&max_v).contains(&&x)
                && (&min_v..=&max_v).contains(&&y)
            {
                count += 1;
            }
        }
    }
    count.into()
}
pub fn apply_gcd(v: &mut [&mut Vec<I>]) {
    println!("START");
    v.iter().for_each(|var| println!("{:?}", var));
    let n_var = v.len();
    let n_val = v[0].len();
    assert!(v.iter().all(|var| var.len() == n_val));
    let gcds = (0..n_val)
        .map(|val_i| {
            (0..n_var)
                .map(|var_i| v[var_i][val_i].clone())
                .reduce(num::integer::gcd)
                .unwrap()
        })
        .collect::<Vec<_>>();
    println!("GCDS");
    println!("{gcds:?}");
    v.iter_mut().for_each(|var| {
        var.iter_mut()
            .zip(gcds.iter())
            .for_each(|(val, gcd)| *val /= gcd)
    });
    println!("END");
    v.iter().for_each(|var| println!("{:?}", var));
}

// p0x+T0*v0x = PX+T0*VX;
// p0y+T0*v0y = PY+T0*VY;
// p0z+T0*v0z = PZ+T0*VZ;
// p1x+T1*v1x = PX+T1*VX;
// p1y+T1*v1y = PY+T1*VY;
// p1z+T1*v1z = PZ+T1*VZ;
//
// T0*(v0x-VX) = PX-p0x
// T0*(v0y-VY) = PY-p0y
//
// 0 = (PX-p0x)*(v0y-VY) - (PY-p0y)*(v0x-VX)
// 0 = (PX-p1x)*(v1y-VY) - (PY-p1y)*(v1x-VX)
// 0 = (PX-p2x)*(v2y-VY) - (PY-p2y)*(v2x-VX)
//
// 0 = v0y*PX+p0x*VY-PX*VY-p0x*v0y - v0x*PY-p0y*VX+VX*PY+p0y*v0x
// 0 = v1y*PX+p1x*VY-PX*VY-p1x*v1y - v1x*PY-p1y*VX+VX*PY+p1y*v1x
//
// 0 = (v0y-v1y)PX + (p0x-p1x)VY + p1x.v1y-p0x.v0y + (v1x-v0x)PY + (p1y-p0y)VX + p0y*v0x-p1y*v1x
// 0 = (v0y-v2y)PX + (p0x-p2x)VY + p2x.v2y-p0x.v0y + (v2x-v0x)PY + (p2y-p0y)VX + p0y*v0x-p2y*v2x
//
//  a0 = (v0y-v1y) ; b0 = (p0x-p1x) ; c0 = v1x-v0x ; d0 = p1y-p0y ; e0 = p1x.v1y-p0x.v0y+p0y.v0x-p1y.v1x
//
//  0 = a0.PX + b0.VY + c0.PY + d0.VX + e0
//  0 = a1.PX + b1.VY + c1.PY + d1.VX + e1
//
//  0 = a1.a0.PX + a1.b0.VY + a1.c0.PY + a1.d0.VX + a1.e0
//  0 = a0.a1.PX + a0.b1.VY + a0.c1.PY + a0.d1.VX + a0.e1
//
// 0 = (a1.b0-a0.b1)VY + (a1.c0-a0.c1)PY + (a1.d0-a0.d1)VX + (a1.e0-a0.e1)
// 0 = (a2.b0-a0.b2)VY + (a2.c0-a0.c2)PY + (a2.d0-a0.d2)VX + (a2.e0-a0.e2)
//
// f0 = a1.b0-a0.b1 ; g0 = a1.c0-a0.c1 ; h0 = a1.d0-a0.d1 ; i0 = a1.e0-a0.e1
//
// 0 = f0.VY + g0.PY + h0.VX + i0
// 0 = f1.VY + g1.PY + h1.VX + i1
//
// 0 = f1.f0.VY + f1.g0.PY + f1.h0.VX + f1.i0
// 0 = f0.f1.VY + f0.g1.PY + f0.h1.VX + f0.i1
//
// 0 = (f1.g0-f0.g1)PY + (f1.h0-f0.h1)VX + (f1.i0-f0.i1)
// 0 = (f2.g0-f0.g2)PY + (f2.h0-f0.h2)VX + (f2.i0-f0.i2)
//
// j0 = f1.g0-f0.g1 ; k0 = f1.h0-f0.h1 ; l0 = f1.i0-f0.i1
//
// 0 = j0.PY + k0.VX + l0
// 0 = j1.PY + k1.VX + l1
//
// 0 = k1.j0.PY + k1.k0.VX + k1.l0
// 0 = k0.j1.PY + k0.k1.VX + k0.l1
//
// 0 = (k1.j0-k0.j1)PY + k1.l0-k0.l1
//
// PY = (k0.l1-k1.l0)/(k1.j0-k0.j1)
// VX = (-l0-j0.PY)/k0
// 0 = f1.f0.VY + f1.g0.PY + f1.h0.VX + f1.i0
// VY = (f1.g0.PY+f1.h0.VX+f1.i0) / -f1.f0
//  0 = a0.PX + b0.VY + c0.PY + d0.VX + e0
//  PX = (b0.VY + c0.PY + d0.VX + e0) / -a0
// p0x+T0*v0x = PX+T0*VX;
// T0 = (PX-p0x)/(v0x-VX)
// T1 = (PX-p1x)/(v1x-VX)
// p0z+T0*v0z = PZ+T0*VZ;
// p1z+T1*v1z = PZ+T1*VZ;
// PZ = (T1.(p0z+T0.v0z)-T0.(p1z+T1*v1z)) / (T1-T0)
#[allow(non_snake_case)]
#[allow(unused_variables)]
fn part2(input: &str) -> I {
    let hailstones = input.lines().map(Hailstone::parse).collect::<Vec<_>>();
    let p = (0..=4)
        .map(|i| Vec3::new(&hailstones[i].p))
        .collect::<Vec<_>>();
    let v = (0..=4)
        .map(|i| Vec3::new(&hailstones[i].v))
        .collect::<Vec<_>>();
    //  a0 = (v0y-v1y) ; b0 = (p0x-p1x) ; c0 = v1x-v0x ; d0 = p1y-p0y ; e0 = p1x.v1y-p0x.v0y+p0y.v0x-p1y.v1x
    let mut a: Vec<_> = (1..=4).map(|z| &v[0].y - &v[z].y).collect();
    let mut b: Vec<_> = (1..=4).map(|z| &p[0].x - &p[z].x).collect();
    let mut c: Vec<_> = (1..=4).map(|z| &v[z].x - &v[0].x).collect();
    let mut d: Vec<_> = (1..=4).map(|z| &p[z].y - &p[0].y).collect();
    let mut e: Vec<_> = (1..=4)
        .map(|z| &p[z].x * &v[z].y - &p[0].x * &v[0].y + &p[0].y * &v[0].x - &p[z].y * &v[z].x)
        .collect();
    apply_gcd(&mut [&mut a, &mut b, &mut c, &mut d, &mut e]);

    // f0 = a1.b0-a0.b1 ; g0 = a1.c0-a0.c1 ; h0 = a1.d0-a0.d1 ; i0 = a1.e0-a0.e1-a2.e0+a0.e2
    let mut f: Vec<_> = (1..=3).map(|z| &a[z] * &b[0] - &a[0] * &b[z]).collect();
    let mut g: Vec<_> = (1..=3).map(|z| &a[z] * &c[0] - &a[0] * &c[z]).collect();
    let mut h: Vec<_> = (1..=3).map(|z| &a[z] * &d[0] - &a[0] * &d[z]).collect();
    let mut i: Vec<_> = (1..=3).map(|z| &a[z] * &e[0] - &a[0] * &e[z]).collect();
    apply_gcd(&mut [&mut f, &mut g, &mut h, &mut i]);

    // j0 = f1.g0-f0.g1 ; k0 = f1.h0-f0.h1 ; l0 = f1.i0-f0.i1
    let mut j: Vec<_> = (1..=2).map(|z| &f[z] * &g[0] - &f[0] * &g[z]).collect();
    let mut k: Vec<_> = (1..=2).map(|z| &f[z] * &h[0] - &f[0] * &h[z]).collect();
    let mut l: Vec<_> = (1..=2).map(|z| &f[z] * &i[0] - &f[0] * &i[z]).collect();
    apply_gcd(&mut [&mut j, &mut k, &mut l]);
    println!("J: {j:?}");
    println!("K: {k:?}");
    println!("L: {l:?}");

    // PY = (k0.l1-k1.l0)/(k1.j0-k0.j1)
    let PY_n = &k[0] * &l[1] - &k[1] * &l[0];
    let PY_d = &k[1] * &j[0] - &k[0] * &j[1];
    println!("PY_n: {PY_n}");
    println!("PY_d: {PY_d}");
    assert_eq!(&PY_n % &PY_d, 0.into());
    let PY = &PY_n / &PY_d;
    println!("PY: {PY}");

    let VX_n = -&l[0] - &j[0] * &PY;
    let VX_d = k[0].clone();
    assert_eq!(&VX_n % &VX_d, 0.into());
    let VX = &VX_n / &VX_d;
    println!("VX: {VX}");

    // VY = (f1.g0.PY+f1.h0.VX+f1.i0) / -f1.f0
    let VY_n = &f[1] * &g[0] * &PY + &f[1] * &h[0] * &VX + &f[1] * &i[0];
    let VY_d = -&f[1] * &f[0];
    assert_eq!(&VY_n % &VY_d, 0.into());
    let VY = &VY_n / &VY_d;
    println!("VY: {VY}");

    //  PX = (b0.VY + c0.PY + d0.VX + e0) / -a0
    let PX_n = &b[0] * &VY + &c[0] * &PY + &d[0] * &VX + &e[0];
    let PX_d = -&a[0];
    assert_eq!(&PX_n % &PX_d, 0.into());
    let PX = PX_n / PX_d;
    println!("PX: {PX}");

    // TO = (PX-p0x)/(v0x-VX)
    let T0_n = &PX - &p[0].x;
    let T0_d = &v[0].x - &VX;
    assert_eq!(&T0_n % &T0_d, 0.into());
    let T0 = T0_n / &T0_d;
    println!("T0: {T0}");
    let T1_n = &PX - &p[1].x;
    let T1_d = &v[1].x - &VX;
    assert_eq!(&T1_n % &T1_d, 0.into());
    let T1 = &T1_n / &T1_d;
    println!("T1: {T1}");

    let PZ_n = &T1 * (&p[0].z + &T0 * &v[0].z) - &T0 * (&p[1].z + &T1 * &v[1].z);
    let PZ_d = &T1 - &T0;
    assert_eq!(&PZ_n % &PZ_d, 0.into());
    let PZ = &PZ_n / &PZ_d;

    println!("PZ: {PZ}");
    PX + PY + PZ
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(
            part1(include_str!("../example.txt"), 7.into(), 27.into()),
            2.into()
        );
    }
    //#[test]
    //fn example_part2() {
    //    assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "7");
    //}
}
