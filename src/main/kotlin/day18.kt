import java.util.*
import kotlin.math.ceil
import kotlin.math.floor

sealed class SnailfishNumber(
    var replaceSelf: ((SnailfishNumber) -> Unit)?,
    private var leafs: LinkedList<Literal>
) {
    class Literal(
        var value: Int,
        replaceSelf: ((SnailfishNumber) -> Unit)?,
        leafs: LinkedList<Literal>
    ) : SnailfishNumber(replaceSelf, leafs)

    class Pair(
        var left: SnailfishNumber,
        var right: SnailfishNumber,
        replaceSelf: ((SnailfishNumber) -> Unit)?,
        leafs: LinkedList<Literal>
    ) : SnailfishNumber(replaceSelf, leafs) {
        init {
            left.replaceSelf = { left = it }
            right.replaceSelf = { right = it }
        }
    }

    operator fun plus(other: SnailfishNumber): SnailfishNumber {
        val leafs = LinkedList<Literal>()
        val new = Pair(this.clone(leafs), other.clone(leafs), null, leafs)
        new.reduce()
        return new
    }

    private fun clone(leafs: LinkedList<Literal>): SnailfishNumber = when (this) {
        is Literal -> {
            val new = Literal(this.value, null, leafs)
            leafs.add(new)
            new
        }
        is Pair -> Pair(left.clone(leafs), right.clone(leafs), null, leafs)
    }

    fun reduce() {
        while (true) {
            if (!(explode(0) || split())) {
                break
            }
        }
    }

    private fun explode(level: Int): Boolean {
        return if (this is Pair) {
            if (level >= 4) {
                val new = Literal(0, replaceSelf, leafs)
                replaceSelf!!(new)

                val leafIndex = leafs.indexOf(left)

                leafs.getOrNull(leafIndex - 1)?.let { it.value += (left as Literal).value }
                leafs.getOrNull(leafIndex + 2)?.let { it.value += (right as Literal).value }

                leafs.removeAt(leafIndex)
                leafs.removeAt(leafIndex)
                leafs.add(leafIndex, new)

                true
            } else {
                left.explode(level + 1) || right.explode(level + 1)
            }
        } else {
            false
        }
    }

    private fun split(): Boolean {
        return when (this) {
            is Pair -> left.split() || right.split()
            is Literal -> {
                return if (value >= 10) {
                    val left = Literal(floor(value / 2.0).toInt(), null, leafs)
                    val right = Literal(ceil(value / 2.0).toInt(), null, leafs)
                    val new = Pair(left, right, replaceSelf, leafs)
                    replaceSelf!!(new)

                    val leafIndex = leafs.indexOf(this)
                    leafs.removeAt(leafIndex)
                    leafs.addAll(leafIndex, listOf(left, right))

                    true
                } else {
                    false
                }
            }
        }
    }

    fun magnitude(): Int {
        return when (this) {
            is Literal -> value
            is Pair -> 3 * left.magnitude() + 2 * right.magnitude()
        }
    }

    override fun toString(): String {
        return when (this) {
            is Literal -> value.toString()
            is Pair -> "[$left,$right]"
        }
    }
}

fun main() {
    fun parse(string: String, leafs: LinkedList<SnailfishNumber.Literal> = LinkedList()): SnailfishNumber {
        return if (string[0] != '[') {
            val new = SnailfishNumber.Literal(string.toInt(), null, leafs)
            leafs.add(new)
            new
        } else {
            var bracketCount = 0
            var commaIndex = 0
            for (i in 1 until string.length) {
                when (string[i]) {
                    '[' -> bracketCount++
                    ']' -> bracketCount--
                    ',' -> {
                        if (bracketCount == 0) {
                            commaIndex = i
                            break
                        }
                    }
                }
            }
            SnailfishNumber.Pair(
                parse(string.substring(1 until commaIndex), leafs),
                parse(string.substring(commaIndex + 1 until string.length - 1), leafs),
                null,
                leafs
            )
        }
    }

    val numbers = input("day18.txt").readLines().map { parse(it) }
    println(numbers.reduce { acc, number -> acc + number }.magnitude())
    println(numbers.maxOf { a -> numbers.filter { b -> a != b }.maxOf { b -> (a + b).magnitude() } })
}
