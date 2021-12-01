fun main() {
    println(input("day01.txt").useLines { part1(it) })
    println(input("day01.txt").useLines { part2(it) })
}

fun part1(input: Sequence<String>): Int {
    return input
        .map { Integer.parseInt(it) }
        .zipWithNext()
        .count { (a, b) -> a < b }
}

fun part2(input: Sequence<String>): Int {
    return input
        .map { Integer.parseInt(it) }
        .windowed(3)
        .map { (a, b, c) -> a + b + c }
        .zipWithNext()
        .count { (a, b) -> a < b }
}
