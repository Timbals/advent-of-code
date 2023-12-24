use itertools::Itertools;
use nalgebra::{matrix, Const, OMatrix, Vector3, Vector6};

pub fn solve_first(input: &str, min: i128, max: i128) -> usize {
    let stones = input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let (x, y, z) = pos
                .split(", ")
                .map(|x| x.trim().parse::<i128>().unwrap())
                .collect_tuple()
                .unwrap();
            let (dx, dy, dz) = vel
                .split(", ")
                .map(|x| x.trim().parse::<i128>().unwrap())
                .collect_tuple()
                .unwrap();
            ((x, y, z), (dx, dy, dz))
        })
        .collect::<Vec<_>>();

    stones
        .iter()
        .tuple_combinations()
        .filter(
            |(((x1, y1, _), (dx1, dy1, _)), ((x3, y3, _), (dx2, dy2, _)))| {
                // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line

                let (x2, y2) = (x1 + dx1, y1 + dy1);
                let (x4, y4) = (x3 + dx2, y3 + dy2);

                let denom1 = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
                let denom2 = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

                if denom1 == 0 || denom2 == 0 {
                    return false;
                }

                let x =
                    ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denom1;
                let y =
                    ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denom2;

                let t1 = (x - x1) / dx1;
                let t2 = (x - x3) / dx2;

                t1 >= 0 && t2 >= 0 && x >= min && x <= max && y >= min && y <= max
            },
        )
        .count()
}

pub fn solve_second(input: &str) -> usize {
    let (
        ((x1, y1, z1), (dx1, dy1, dz1)),
        ((x2, y2, z2), (dx2, dy2, dz2)),
        ((x3, y3, z3), (dx3, dy3, dz3)),
    ) = input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let (x, y, z) = pos
                .split(", ")
                .map(|x| x.trim().parse::<isize>().unwrap() as f64)
                .collect_tuple()
                .unwrap();
            let (dx, dy, dz) = vel
                .split(", ")
                .map(|x| x.trim().parse::<isize>().unwrap() as f64)
                .collect_tuple()
                .unwrap();
            ((x, y, z), (dx, dy, dz))
        })
        .take(3)
        .collect_tuple()
        .unwrap();

    // known: x1, dx1, y1, dy1, z1, dz1, x2, dx2, y2, dy2, z2, dz2, x3, dx3, y3, dy3, z3, dz3
    // unknown: x, y, z, dx, dy, dz, t1, t2, t3
    // x + t1 * dx = x1 + t1 * dx1
    // y + t1 * dy = y1 + t1 * dy1
    // z + t1 * dz = z1 + t1 * dz1
    //
    // x + t2 * dx = x2 + t2 * dx2
    // y + t2 * dy = y2 + t2 * dy2
    // z + t2 * dz = z2 + t2 * dz2
    //
    // x + t3 * dx = x3 + t3 * dx3
    // y + t3 * dy = y3 + t3 * dy3
    // z + t3 * dz = z3 + t3 * dz3
    //
    //
    //    p + t1 * d          = p1 + t1 * d1
    // => (p - p1)            = -t1 * (d - d1)     (co-linear vectors -> cross product is 0)
    // => (p - p1)            = -t1 * (d - d1)
    // => (p - p1) x (d - d1) = 0
    //
    // (p - p1) x (d - d1) = 0
    // =>
    // (p - p1) x (d - d1) = (p - p2) x (d - d2)
    // =>
    // (y - y1) * (dz - dz1) - (z - z1) * (dy - dy1) = 0
    // (z - z1) * (dx - dx1) - (x - x1) * (dz - dz1) = 0
    // (x - x1) * (dy - dy1) - (y - y1) * (dx - dx1) = 0
    // =>
    // (y - y1) * (dz - dz1) - (z - z1) * (dy - dy1) = 0
    // y * dz - y * dz1 - y1 * dz + y1 - y1 * dz1 - z * dy + z * dy1 + z1 * dy - z1 * dy1 = y * dz - y * dz2 - y2 * dz + y2 - y2 * dz2 - z * dy + z * dy2 + z2 * dy - z2 * dy2
    // - y * dz1 - y1 * dz + y1 - y1 * dz1 + z * dy1 + z1 * dy - z1 * dy1 = - y * dz2 - y2 * dz + y2 - y2 * dz2 + z * dy2 + z2 * dy - z2 * dy2
    // y * (dz2 - dz1)
    //
    // (p - p1) x (d - d1) = 0
    // p x d - p x d1 - p1 x d + p1 x d1 = 0
    // p x d - p x d1 - p1 x d + p1 x d1 = p x d - p x d2 - p2 x d + p2 x d2
    // - p x d1 - p1 x d + p1 x d1 = - p x d2 - p2 x d + p2 x d2
    // - p x d1 - p1 x d + p x d2 + p2 x d = p2 x d2 - p1 x d1
    // p x (d2 - d1) + d x (p2 - p1) = p2 x d2 - p1 x d1
    //
    // Use
    // p x (d2 - d1) + d x (p2 - p1) = p2 x d2 - p1 x d1
    // and
    // p x (d3 - d2) + d x (p3 - p2) = p3 x d3 - p2 x d2
    // to construct a linear system of equations

    let p1 = Vector3::new(x1, y1, z1);
    let p2 = Vector3::new(x2, y2, z2);
    let p3 = Vector3::new(x3, y3, z3);

    let d1 = Vector3::new(dx1, dy1, dz1);
    let d2 = Vector3::new(dx2, dy2, dz2);
    let d3 = Vector3::new(dx3, dy3, dz3);

    let cross_matrix = |v: Vector3<f64>| {
        matrix![
             0.0,  v.z, -v.y;
            -v.z,  0.0,  v.x;
             v.y, -v.x,  0.0;
        ]
    };

    let b1 = p2.cross(&d2) - p1.cross(&d1);
    let b2 = p3.cross(&d3) - p2.cross(&d2);

    let b = Vector6::new(b1.x, b1.y, b1.z, b2.x, b2.y, b2.z);
    let mut a = OMatrix::<f64, Const<6>, Const<6>>::zeros();
    a.fixed_view_mut::<3, 3>(0, 0)
        .copy_from(&cross_matrix(d2 - d1));
    a.fixed_view_mut::<3, 3>(0, 3)
        .copy_from(&cross_matrix(p2 - p1));
    a.fixed_view_mut::<3, 3>(3, 0)
        .copy_from(&cross_matrix(d3 - d2));
    a.fixed_view_mut::<3, 3>(3, 3)
        .copy_from(&cross_matrix(p3 - p2));

    let x = a.lu().solve(&b).unwrap();

    x.fixed_rows::<3>(0).sum() as usize
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(2, solve_first(sample, 7, 27));
    assert_eq!(47, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(17776, solve_first(input, 200000000000000, 400000000000000));
    assert_eq!(948978092202212, solve_second(input));
}
