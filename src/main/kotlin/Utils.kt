import java.io.File

fun input(path: String): File {
    return File(ClassLoader.getSystemResource(path).toURI())
}
