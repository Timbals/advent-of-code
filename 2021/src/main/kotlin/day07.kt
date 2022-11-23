import kotlin.math.*

fun main() {
    val crabs = input("day07.txt").readLines().first().split(",").map(Integer::parseInt)

    val median = crabs.sorted()[crabs.size / 2]
    val totalFuelPart1 = crabs.sumOf { abs(it - median) }
    println(totalFuelPart1)

    val average = crabs.average()
    fun fuel(pos: Int) = crabs.sumOf { (1..abs(it - pos)).sum() }
    val totalFuelPart2 = min(fuel(ceil(average).toInt()), fuel(floor(average).toInt()))
    println(totalFuelPart2)
}
