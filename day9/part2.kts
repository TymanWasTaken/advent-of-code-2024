#!/usr/bin/env kotlin

import java.io.File
import java.util.*

val debug = true

val disk = File(if (args.size >= 1 && args[0] == "example") "./example.txt" else "./input.txt").readLines()[0]

// Parse into labeled sections
var id = 0
val parsed = disk.flatMapIndexed { i, char -> // I love linked lists i love linked lists i love linked lists
    val num = char.digitToInt()

    if (i % 2 == 0) {
        // Every other starting with i = 0 should be real data
        List(num) { i / 2 }
    } else {
        List(num) { null }
    }
}.toMutableList()

var iter = parsed.asReversed().listIterator()
println(parsed.asReversed())
while (true) {
    var startOfGroup = iter.next()
    while (startOfGroup == null) {
        startOfGroup = iter.next()
    }
    println(iter.nextIndex() - 1)
    var endOfGroup = iter.next()
    while (endOfGroup == startOfGroup) {
        endOfGroup = iter.next()
    }
    iter.previous()
    println(iter.nextIndex() - 1)
    break


    var emptySpaceBuffer = 0
    for (i in parsed.indices) {
        if (parsed[i] == null) emptySpaceBuffer += 1
        else emptySpaceBuffer = 0

//        if (emptySpaceBuffer == )
    }
}