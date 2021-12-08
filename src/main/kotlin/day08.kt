fun main() {
    fun part1(lines: List<String>) =
        lines.map { it.split(" | ")[1].split(" ") }.flatten().count { it.length in setOf(2, 4, 3, 7) }

    fun part2(lines: List<String>) = lines.sumOf { line ->
        val (digits, output) = line.split(" | ").map { it.split(" ") }.firstLast()

        val segments = Array(10, init = { emptySet<Char>() })
        fun findSegments(predicate: (String) -> Boolean) = digits.find(predicate)?.toSet() ?: emptySet()

        segments[1] = findSegments { it.length == 2 }
        segments[4] = findSegments { it.length == 4 }
        segments[7] = findSegments { it.length == 3 }
        segments[8] = findSegments { it.length == 7 }
        segments[6] = findSegments { it.length == 6 && it.toSet().intersect(segments[1]).size == 1 }
        segments[9] = findSegments { it.length == 6 && it.toSet().intersect(segments[4]).size == 4 }
        segments[0] = findSegments { it.length == 6 && it.toSet() != segments[6] && it.toSet() != segments[9] }
        segments[2] = findSegments { it.length == 5 && it.toSet().intersect(segments[4]).size == 2 }
        segments[3] = findSegments { it.length == 5 && it.toSet().intersect(segments[1]).size == 2 }
        segments[5] = findSegments { it.length == 5 && (it.toSet() intersect segments[6]).size == 5 }

        output.joinToString(separator = "") { segments.indexOf(it.toSet()).toString() }.toInt()
    }

    val lines = input("day08.txt").readLines()

    println(part1(lines))
    println(part2(lines))
}
