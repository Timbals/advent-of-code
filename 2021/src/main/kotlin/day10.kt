fun main() {
    fun part1(lines: List<String>): Long = lines.sumOf { line ->
        val stack = ArrayDeque<Char>()

        when (line.firstOrNull {
            !when (it) {
                '(' -> stack.add(')')
                '[' -> stack.add(']')
                '{' -> stack.add('}')
                '<' -> stack.add('>')
                else -> it == stack.removeLast()
            }
        }) {
            ')' -> 3
            ']' -> 57
            '}' -> 1197
            '>' -> 25137
            else -> 0L
        }
    }

    fun part2(lines: List<String>): Long = lines.mapNotNull { line ->
        val stack = ArrayDeque<Char>()

        if (line.all {
                when (it) {
                    '(' -> stack.add(')')
                    '[' -> stack.add(']')
                    '{' -> stack.add('}')
                    '<' -> stack.add('>')
                    else -> it == stack.removeLast()
                }
            }) {

            stack.map {
                when (it) {
                    ')' -> 1
                    ']' -> 2
                    '}' -> 3
                    '>' -> 4
                    else -> throw IllegalArgumentException("invalid stack")
                }
            }.reversed().fold(0L) { acc, i -> acc * 5 + i }
        } else {
            null
        }
    }.sorted().run { get(size / 2) }

    val lines = input("day10.txt").readLines()

    println(part1(lines))
    println(part2(lines))
}

