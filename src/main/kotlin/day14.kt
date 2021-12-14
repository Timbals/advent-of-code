import java.math.BigInteger

fun main() {
    val lines = input("day14.txt").readLines()

    val template = lines[0]

    val rules = Array(26) { Array<Char?>(26) { null } }
    for (rule in lines.subList(2, lines.size)) {
        val (from, to) = rule.split(" -> ")
        rules[from.first().code - 65][from.last().code - 65] = to.first()
    }

    fun solve(steps: Int): BigInteger {
        val cache = Array(26) { Array(26) { Array(steps) { emptyMap<Char, BigInteger>() } } }

        /**
         * Counts the occurrences of characters when the String "$first$second" is expanded for level more steps.
         * Uses a cache to memoize values.
         */
        fun countOccurrences(first: Char, second: Char, level: Int): Map<Char, BigInteger> {
            if (level == 0) {
                return if (first == second) mapOf(first to BigInteger.TWO) else mapOf(
                    first to BigInteger.ONE,
                    second to BigInteger.ONE
                )
            }

            return cache[first.code - 65][second.code - 65][level - 1].ifEmpty {
                val map = mutableMapOf<Char, BigInteger>()

                rules[first.code - 65][second.code - 65]?.also { rule ->
                    map.mergeMap(countOccurrences(first, rule, level - 1), BigInteger::plus)
                    map.mergeMap(countOccurrences(rule, second, level - 1), BigInteger::plus)
                    map.merge(rule, BigInteger.ONE, BigInteger::minus)
                }

                cache[first.code - 65][second.code - 65][level - 1] = map

                map
            }
        }

        val map = mutableMapOf<Char, BigInteger>()
        for (i in 0 until template.length - 1) {
            map.mergeMap(countOccurrences(template[i], template[i + 1], steps), BigInteger::plus)
        }

        return map.values.maxOrNull()!! - map.values.minOrNull()!!
    }

    println(solve(10))
    println(solve(40))
}
