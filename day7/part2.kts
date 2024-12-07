#!/usr/bin/env kotlin

import java.io.File
import java.time.Instant
import java.util.*
import kotlin.math.*

infix fun Long.`||`(other: Long)
        = ((this * 10.0.pow(floor(log10(other.toDouble())) + 1)) + other).toLong()

val start = Instant.now()

val tests = File("./input.txt").readLines().map {
    Pair(it.split(": ")[0], it.split(": ")[1])
}.map {
    Pair(it.first.toLong(), it.second.split(" ").map { n -> n.toLong() })
}

var finalResult = 0L

tests.forEach { (test, values) ->
    var results = LinkedList<Long>()
    results.add(values[0])

    for (i in 1..<values.size) {
        val iter = results.listIterator();
        while (iter.hasNext()) {
            val result = iter.next()

            iter.set(result * values[i])
            iter.add(result + values[i])
            iter.add(result `||` values[i])
        }
    }

    if (test in results) {
        finalResult += test
    }
}

val time = (Instant.now().toEpochMilli() - start.toEpochMilli())

println("Result: ${finalResult}\nTime: ${time}ms")
