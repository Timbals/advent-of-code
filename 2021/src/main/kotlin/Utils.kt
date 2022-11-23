import java.io.File
import java.util.function.BiFunction

fun input(path: String): File {
    return File(ClassLoader.getSystemResource(path).toURI())
}

fun <T> List<List<T>>.transpose(): List<List<T>> =
    fold(first().map(::listOf)) { acc, next -> acc.zip(next).map { (list, element) -> list + element } }

fun <T> List<T>.firstLast(): Pair<T, T> = Pair(first(), last())

fun <K, V> MutableMap<K, V>.mergeMap(other: Map<K, V>, operation: BiFunction<in V, in V, out V?>) {
    other.forEach { entry -> this.merge(entry.key, entry.value!!, operation) }
}
