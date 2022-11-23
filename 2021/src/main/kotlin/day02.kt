fun main() {
    val lines = input("day02.txt").readLines()
    println("${part1(lines)}")
    println("${part2(lines)}")
}

fun part1(lines: List<String>): Int {
    var horizontal = 0
    var depth = 0

    for (line in lines) {
        val (command, value) = line.split(" ")
        val x = value.toInt()
        when (command) {
            "forward" -> horizontal += x
            "down" -> depth += x
            "up" -> depth -= x
        }
    }

    return horizontal * depth
}

fun part2(lines: List<String>): Int {
    var aim = 0
    var horizontal = 0
    var depth = 0

    for (line in lines) {
        val (command, value) = line.split(" ")
        val x = value.toInt()
        when (command) {
            "forward" -> {
                horizontal += x
                depth += aim * x
            }
            "down" -> aim += x
            "up" -> aim -= x
        }
    }

    return horizontal * depth
}
