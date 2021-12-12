fun main() {
    val lines = input("day12.txt").readLines()

    val edges = mutableMapOf<String, MutableList<String>>()
    for (line in lines) {
        val (a, b) = line.split("-")
        edges.computeIfAbsent(a) { mutableListOf() }.add(b)
        edges.computeIfAbsent(b) { mutableListOf() }.add(a)
    }
    edges.forEach { it.value.remove("start") }
    edges.remove("end")

    fun solve(part1: Boolean): Int {
        val paths = mutableListOf(Pair(listOf("start"), false))
        var count = 0

        while (paths.isNotEmpty()) {
            val (path, visitedSmallTwice) = paths.removeFirst()

            if (path.last() == "end") {
                count++
            } else {
                for (node in edges[path.last()]!!) {
                    if (node.first().isLowerCase() && path.contains(node)) {
                        if (visitedSmallTwice) {
                            continue
                        } else if (!part1) {
                            paths.add(Pair(path + node, true))
                        }
                    } else {
                        paths.add(Pair(path + node, visitedSmallTwice))
                    }
                }
            }
        }

        return count
    }

    println(solve(true))
    println(solve(false))
}
