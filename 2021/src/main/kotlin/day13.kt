fun main() {
    val lines = input("day13.txt").readLines()

    val split = lines.indexOf("")

    val maxX = lines.subList(0, split).maxOf { it.split(",")[0].toInt() }
    val maxY = lines.subList(0, split).maxOf { it.split(",")[1].toInt() }
    val values = Array(maxX + 1) { Array(maxY + 1) { false } }

    for (line in lines.subList(0, split)) {
        val (x, y) = line.split(",")
        values[x.toInt()][y.toInt()] = true
    }

    var smallestXFold = maxX
    var smallestYFold = maxY
    var firstFold = true

    for (line in lines.subList(split + 1, lines.size)) {
        val (xy, foldStr) = line.split("=")
        val fold = foldStr.toInt()
        if (xy.last() == 'x') {
            smallestXFold = fold
            for (x in fold..maxX) {
                for (y in 0..maxY) {
                    if (values[x][y]) {
                        values[x][y] = false
                        values[fold - (x - fold)][y] = true
                    }
                }
            }
        } else {
            smallestYFold = fold
            for (x in 0..maxX) {
                for (y in fold..maxY) {
                    if (values[x][y]) {
                        values[x][y] = false
                        values[x][fold - (y - fold)] = true
                    }
                }
            }
        }

        if (firstFold) {
            firstFold = false
            println(values.sumOf { column -> column.count { it } })
        }
    }

    for (y in 0 until smallestYFold) {
        for (x in 0 until smallestXFold) {
            print(if (values[x][y]) "█" else " ")
        }
        println()
    }
}
