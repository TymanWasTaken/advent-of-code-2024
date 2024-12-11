#!/usr/bin/env kotlin

import java.io.File
import java.util.*
import kotlin.math.min

data class ReplicatedData(
    var fileId: Int?,
    var times: Int,
)

val disk = File(if (args.size >= 1 && args[0] == "example") "./example.txt" else "./input.txt").readLines()[0]

// Parse into labeled sections
var id = 0
val parsed = LinkedList(disk.mapIndexed { i, char -> // I love linked lists i love linked lists i love linked lists
    val num = char.digitToInt()

    if (i % 2 == 0) {
        // Every other starting with i = 0 should be real data
        ReplicatedData(
            fileId = i / 2,
            times = num
        )
    } else {
        ReplicatedData(
            fileId = null,
            times = num
        )
    }
}.filter { it.times != 0 })

val iter = parsed.listIterator()
var blocksFilled = 0

while (iter.hasNext()) {
    val currentGroup = iter.next()
    if (currentGroup.fileId != null || currentGroup.times < 1) continue

    val (groupTakingFromIndex, groupTakingFrom) = parsed.asReversed()
        .filter {it.times > 0 }
        .withIndex()
        .find { (_, it) -> it.fileId != null } ?: break
    if (parsed.size - 1 - groupTakingFromIndex <= iter.nextIndex() - 1) break

    val amountToMove = min(currentGroup.times, groupTakingFrom.times)

    // Take the amount we are moving from the previous location
    groupTakingFrom.times -= amountToMove
    // And add it before the current one
    val originalEmptySpace = currentGroup.times
    iter.set(
        ReplicatedData(
            fileId = groupTakingFrom.fileId,
            times = amountToMove
        )
    )
    blocksFilled += amountToMove
    // If we still have some left, insert back in the empty space and rewind the cursor
    if (amountToMove < originalEmptySpace) {
        iter.add(
            ReplicatedData(
                fileId = null,
                times = originalEmptySpace - amountToMove
            )
        )
        iter.previous() // Go back one so the next iteration of the loop will catch the new empty data we just added
    }
}

println(
    parsed
        .filter { it.times > 0 && it.fileId != null }
        .flatMap { group -> (1..group.times).map { _ -> group.fileId!! } }
        .mapIndexed { i, data -> i.toULong() * data.toULong() }.sum()
)