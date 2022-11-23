import kotlin.math.max
import kotlin.math.min

fun main() {
    val lines = input("day16.txt").readLines()

    val packet =
        lines.first().map { it.digitToInt(16).toString(2).padStart(4, '0') }.joinToString("").iterator()

    var progress = 0
    fun CharIterator.next(n: Int) = (1..n).map {
        progress++
        next()
    }.joinToString("")

    var totalVersion = 0

    fun parsePacket(): Long {
        val packetVersion = packet.next(3).toInt(2)
        val packetType = packet.next(3).toInt(2)

        totalVersion += packetVersion

        return if (packetType == 4) {
            // literal packet
            var binary = ""
            while (packet.next(1) == "1") {
                binary += packet.next(4)
            }
            binary += packet.next(4)

            binary.toLong(2)
        } else {
            // operator packet
            val operands = mutableListOf<Long>()

            val lengthType = packet.next(1)
            if (lengthType == "0") {
                val subPacketsLength = packet.next(15).toInt(2)
                val end = progress + subPacketsLength
                while (progress != end) {
                    operands.add(parsePacket())
                }
            } else {
                val subPacketCount = packet.next(11).toInt(2)
                for (i in 1..subPacketCount) {
                    operands.add(parsePacket())
                }
            }

            val operation: (Long, Long) -> Long = when (packetType) {
                0 -> { a, b -> a + b }
                1 -> { a, b -> a * b }
                2 -> { a, b -> min(a, b) }
                3 -> { a, b -> max(a, b) }
                5 -> { a, b -> if (a > b) 1 else 0 }
                6 -> { a, b -> if (a < b) 1 else 0 }
                7 -> { a, b -> if (a == b) 1 else 0 }
                else -> throw IllegalArgumentException("invalid packet type id")
            }

            operands.reduce(operation)
        }
    }

    val result = parsePacket()
    println(totalVersion)
    println(result)
}
