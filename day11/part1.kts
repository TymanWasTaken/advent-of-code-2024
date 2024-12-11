#!/usr/bin/env kotlin

import java.io.File
import java.util.LinkedList
import kotlin.math.log10

val stones = LinkedList(File("input.txt").readLines()[0].split(" ").map { it.toLong() })

for (i in 0..<25) {
    val iter = stones.listIterator()

    while (iter.hasNext()) {
        val stone = iter.next()

        if (stone == 0L) {
            iter.set(1)
        } else if ((log10(stone.toDouble()).toInt() + 1) % 2 == 0) {
            val str = stone.toString()
            iter.set(str.substring(0, str.length / 2).toLong())
            iter.add(str.substring(str.length / 2, str.length).toLong())
        } else {
            iter.set(stone * 2024)
        }
    }
}

println("Result: ${stones.size}")