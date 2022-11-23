fun main() {
    fun part1(lines: List<String>): Int {
        var gamma = ""
        var epsilon = ""

        for (index in (0 until (lines.firstOrNull()?.length ?: 0))) {
            if (lines.count { it[index] == '0' } > lines.size / 2) {
                gamma += '0'
                epsilon += '1'
            } else {
                gamma += '1'
                epsilon += '0'
            }
        }

        return Integer.parseInt(gamma, 2) * Integer.parseInt(epsilon, 2)
    }

    fun part2(lines: List<String>): Int {
        var oxygen = 0
        var oxygenLines = lines
        var co2 = 0
        var co2Lines = lines

        for (index in (0 until (lines.firstOrNull()?.length ?: 0))) {
            oxygen = oxygenLines.lastOrNull()?.toInt(2) ?: oxygen

            oxygenLines = if (oxygenLines.count { it[index] == '0' } > oxygenLines.size / 2) {
                oxygenLines.filter { it[index] == '0' }
            } else {
                oxygenLines.filter { it[index] == '1' }
            }

            co2 = co2Lines.lastOrNull()?.toInt(2) ?: co2

            co2Lines = if (co2Lines.count { it[index] == '0' } > co2Lines.size / 2) {
                co2Lines.filter { it[index] == '1' }
            } else {
                co2Lines.filter { it[index] == '0' }
            }
        }

        return oxygen * co2
    }

    val lines = input("day03.txt").readLines()
    println(part1(lines))
    println(part2(lines))
}
