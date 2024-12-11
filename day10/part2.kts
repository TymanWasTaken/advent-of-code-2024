#!/usr/bin/env kotlin

import java.io.File

operator fun <E> List<List<E>>.get(position: Pair<Int, Int>): E = this[position.first][position.second]

val trailheads = mutableListOf<Pair<Int, Int>>()
val grid = File("input.txt").readLines().mapIndexed { row, line ->
    line.toCharArray().mapIndexed { col, char ->
        if (char == '0') trailheads.add(row to col)
        char.digitToInt()
    }
}

var sum = 0
for (trailhead in trailheads) {
    val positions = mutableListOf(trailhead)

    while (!positions.all { pos -> grid[pos] == 9 }) {
        val iter = positions.listIterator()
        while (iter.hasNext()) {
            val pos = iter.next()

            iter.remove()
            if (grid[pos] == 9) {
                break
            }

            if (grid[pos.first].getOrNull(pos.second - 1) == grid[pos] + 1) iter.add(pos.first to pos.second - 1)
            if (grid[pos.first].getOrNull(pos.second + 1) == grid[pos] + 1) iter.add(pos.first to pos.second + 1)
            if (grid.getOrNull(pos.first + 1)?.get(pos.second) == grid[pos] + 1) iter.add(pos.first + 1 to pos.second)
            if (grid.getOrNull(pos.first - 1)?.get(pos.second) == grid[pos] + 1) iter.add(pos.first - 1 to pos.second)
        }
    }

    val tops = positions.size
    sum += tops
}

println("Result: $sum")