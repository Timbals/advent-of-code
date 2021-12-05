import java.io.File

fun input(path: String): File {
    return File(ClassLoader.getSystemResource(path).toURI())
}

fun <T> List<List<T>>.transpose(): List<List<T>> {
    return fold(first().map(::listOf)) { acc, next -> acc.zip(next).map { (list, element) -> list + element } }
}
