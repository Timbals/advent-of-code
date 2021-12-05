import kotlin.math.max
import kotlin.math.min

fun main() {
    val lines = input("day05.txt").readLines()

    fun overlappingCount(diagonals: Boolean): Int {
        val occurrences = Array(1000) { IntArray(1000) }

        for (line in lines) {
            val (first, second) = line.split(" -> ").firstLast()
            val (x1, y1) = first.split(",").map(Integer::parseInt).firstLast()
            val (x2, y2) = second.split(",").map(Integer::parseInt).firstLast()

            if (x1 == x2 || y1 == y2) {
                // horizontal and vertical lines
                for (y in min(y1, y2)..max(y1, y2)) {
                    for (x in min(x1, x2)..max(x1, x2)) {
                        occurrences[x][y] += 1
                    }
                }
            } else if (diagonals) {
                // diagonal lines
                for ((x, y) in ((x1..x2) + (x1 downTo x2)) zip ((y1..y2) + (y1 downTo y2))) {
                    occurrences[x][y] += 1
                }
            }
        }

        return occurrences.sumOf { row -> row.count { it > 1 } }
    }

    println(overlappingCount(false))
    println(overlappingCount(true))
}
