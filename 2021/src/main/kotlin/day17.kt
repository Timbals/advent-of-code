import kotlin.math.max
import kotlin.math.min

fun main() {
    val match =
        Regex("target area: x=(-?\\d+)..(-?\\d+), y=(-?\\d+)..(-?\\d+)").find(input("day17.txt").readLines().first())!!
    val (x1, x2, y1, y2) = match.destructured
    val targetX = x1.toInt()..x2.toInt()
    val targetY = y1.toInt()..y2.toInt()

    val times = (0..targetX.last).associateWith { t ->
        (1..targetX.last).filter { v -> min(t, v + 1) * (max(v - t + 1, 0) + v) / 2 in targetX }.toSet()
    }.filter { it.value.isNotEmpty() }
    val infiniteVelocities = (1..targetX.last).filter { (it + 1) * it / 2 in targetX }

    var count = 0

    val pairs = mutableSetOf<Pair<Int, Int>>()
    var highestY = 0

    // no idea what the real bound is, so I just choose 10000
    for (vel in targetY.first..10000) {
        var y = 0
        var highestForVel = y
        var v = vel
        var t = 0
        while (y >= targetY.first) {
            y += v--
            t++
            highestForVel = max(y, highestForVel)
            if (y in targetY && (t in times || t >= targetX.last)) {
                val xValues = if (t >= targetX.last) infiniteVelocities else times[t]!!
                count += xValues.size
                for (x in xValues) {
                    pairs.add(Pair(x, vel))
                }

                highestY = max(highestY, highestForVel)
            }
        }
    }

    println(highestY)
    println(pairs.size)
}
