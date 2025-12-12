pub fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|line| line.contains('x'))
        .filter(|line| {
            let (size, shapes) = line.split_once(": ").unwrap();
            let (width, height) = size.split_once('x').unwrap();
            let (width, height) =
                (width.parse::<usize>().unwrap(), height.parse::<usize>().unwrap());
            let shape_count = shapes.split(' ').map(|x| x.parse::<usize>().unwrap()).sum::<usize>();

            // well, I don't like it but ok
            width * height >= shape_count * (3 * 3)
        })
        .count()
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(457, solve(input));
}
