import java.util.*

fun main() {
    val lines = input("day15.txt").readLines()

    println(solve(lines))
    println(solve(lines, 5))
}

fun solve(lines: List<String>, tiles: Int = 1): Int? {
    val columns = lines.size
    val rows = lines[0].length

    val grid = Array(columns * tiles) { column ->
        Array(rows * tiles) { row ->
            if (column < columns && row < rows) {
                lines[column][row].digitToInt()
            } else {
                val tileDistance = column / columns + row / rows
                (lines[column % columns][row % rows].digitToInt() + tileDistance - 1) % 9 + 1
            }
        }
    }

    // dijkstra
    val queue = PriorityQueue<Triple<Int, Int, Int>> { a, b -> a.first - b.first }
    val visited = Array(columns * tiles) { Array(rows * tiles) { false } }
    queue.add(Triple(0, 0, 0))

    while (queue.isNotEmpty()) {
        val (cost, column, row) = queue.remove()!!

        if (column == grid.size - 1 && row == grid[0].size - 1) {
            return cost
        }

        if (visited[column][row]) {
            continue
        } else {
            visited[column][row] = true
        }

        val neighbors = listOf(
            Pair(column + 1, row),
            Pair(column - 1, row),
            Pair(column, row + 1),
            Pair(column, row - 1),
        )

        for (neighbor in neighbors.filterNot { visited.getOrNull(it.first)?.getOrNull(it.second) ?: true }) {
            queue.add(Triple(cost + grid[neighbor.first][neighbor.second], neighbor.first, neighbor.second))
        }
    }

    return null
}
