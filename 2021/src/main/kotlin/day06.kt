import java.math.BigInteger

fun main() {
    fun populationMap(days: Int, reset: Int = 6, new: Int = 8): Map<Int, BigInteger> {
        val populationMap = mutableMapOf<Int, BigInteger>()

        for (i in 0..new) {
            var population = Array<BigInteger>(new + 1) { BigInteger.ZERO }
            population[new] = BigInteger.ONE
            for (day in 1..days + new) {
                population = Array(new + 1) {
                    when (it) {
                        reset -> population[it + 1] + population[0]
                        new -> population[0]
                        else -> population[it + 1]
                    }
                }

                if (day >= days) {
                    populationMap[new - day + days] = population.sumOf { it }
                }
            }
        }

        return populationMap
    }

    val initial = input("day06.txt").readLines().first().split(",")
    println(initial.map(Integer::parseInt).sumOf { populationMap(80)[it] ?: BigInteger.ZERO })
    println(initial.map(Integer::parseInt).sumOf { populationMap(256)[it] ?: BigInteger.ZERO })
}
