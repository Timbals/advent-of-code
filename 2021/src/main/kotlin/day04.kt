import kotlin.math.min

typealias Board = List<List<Pair<Int, Int>>>

fun main() {
    val lines = input("day04.txt").readLines()
    val draws = lines.first().split(",").map(Integer::parseInt)

    fun solveBingo(win: Boolean) = lines.drop(1).chunked(6).map { board ->
        val rows = board
            .drop(1)
            .map { it.trim().split(Regex(" +")) }
            .map { it.map(Integer::parseInt).map { number -> Pair(number, draws.indexOf(number)) } }

        fun Board.firstCompleted() = minOfOrNull { column -> column.maxOf { it.second } } ?: Int.MAX_VALUE
        val firstCompleted = min(rows.firstCompleted(), rows.transpose().firstCompleted())

        val sum = rows.flatten().filter { it.second > firstCompleted }.sumOf { it.first }
        val score = sum * draws[firstCompleted]
        Pair(firstCompleted, score)
    }.minByOrNull { if (win) it.first else -it.first }?.second

    println(solveBingo(true))
    println(solveBingo(false))
}
