fun main() {
    fun lowPoints(lines: List<String>): List<Pair<Int, Int>> {
        val lowPoints = mutableListOf<Pair<Int, Int>>()

        for (y in lines.indices) {
            for (x in lines[0].indices) {
                val height = lines[y][x].digitToInt()
                val adjacent = listOfNotNull(
                    lines.getOrNull(y + 1)?.get(x),
                    lines.getOrNull(y - 1)?.get(x),
                    lines[y].getOrNull(x + 1),
                    lines[y].getOrNull(x - 1)
                )
                if (adjacent.all { height < it.digitToInt() }) {
                    lowPoints.add(Pair(y, x))
                }
            }
        }

        return lowPoints
    }

    fun part1(lines: List<String>) = lowPoints(lines).sumOf { lines[it.first][it.second].digitToInt() + 1 }

    fun part2(lines: List<String>) = lowPoints(lines).map { lowPoint ->
        // naive flood-fill
        val queue = ArrayDeque<Pair<Int, Int>>()
        queue.add(lowPoint)
        val visited = mutableSetOf(lowPoint)

        while (queue.isNotEmpty()) {
            val (y, x) = queue.removeFirst()
            val height = lines[y][x].digitToInt()

            val adjacent = listOf(
                Pair(y + 1, x), Pair(y - 1, x), Pair(y, x + 1), Pair(y, x - 1)
            )

            for (next_node in adjacent.filter {
                val localHeight = lines.getOrNull(it.first)?.getOrNull(it.second)?.digitToInt()
                localHeight != null && localHeight != 9 && localHeight >= height && !visited.contains(it)
            }) {
                queue.add(next_node)
                visited.add(next_node)
            }
        }

        visited.size
    }.sortedDescending().take(3).reduce { acc, i -> acc * i }

    val lines = input("day09.txt").readLines()

    println(part1(lines))
    println(part2(lines))
}
