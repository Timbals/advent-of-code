fun main() {
    val energy = input("day11.txt").readLines().map { row -> row.map { it.digitToInt() }.toMutableList() }

    fun flashing(y: Int, x: Int) = energy[y][x] == -1

    fun flash(y: Int, x: Int) {
        energy[y][x] = -1

        for (offsetY in -1..1) {
            for (offsetX in -1..1) {
                val newY = y + offsetY
                val newX = x + offsetX
                if (offsetX == 0 && offsetY == 0) {
                    continue
                }
                if (newY < 0 || newY >= energy.size) {
                    continue
                }
                if (newX < 0 || newX >= energy[newY].size) {
                    continue
                }
                if (energy[newY][newX] == -1) {
                    continue
                }

                energy[newY][newX]++

                if (energy[newY][newX] > 9) {
                    flash(newY, newX)
                }
            }
        }
    }

    var step = 0
    var totalFlashing = 0
    while (!energy.all { row -> row.all { it == -1 } }) {
        if (step <= 100) totalFlashing += energy.sumOf { row -> row.count { it == -1 } }
        energy.forEachIndexed { y, row -> row.indices.forEach { x -> if (flashing(y, x)) energy[y][x] = 0 } }

        for (y in energy.indices) {
            for (x in energy[y].indices) {
                if (flashing(y, x)) {
                    continue
                }

                energy[y][x]++

                if (energy[y][x] > 9) {
                    flash(y, x)
                }
            }
        }

        step++
    }

    println(totalFlashing)
    println(step)
}
