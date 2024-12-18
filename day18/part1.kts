#!/usr/bin/env kotlin

import java.io.File
import java.util.LinkedList

data class Block (
    var type: Type,
    var seen: Boolean = false,
    var distance: Int = Int.MAX_VALUE
) {
    enum class Type {
        Empty,
        Byte,
    }
}

val bytes = File("input.txt").readLines().subList(0, 1024).map { it.split(",").let { s -> s[0].toInt() to s[1].toInt() } }.toSet()
val grid = (0..70).map { row -> (0..70).map { col ->
    Block(
        type = if (bytes.contains(col to row)) Block.Type.Byte
               else Block.Type.Empty,
    )
}.toMutableList() }.toMutableList()
grid[0][0].seen = true
grid[0][0].distance = 0

val queue = LinkedList<Pair<Int, Int>>()
queue.addFirst(0 to 0)

while (!queue.isEmpty()) {
    // Take one off the queue
    val next = queue.pollLast() ?: break

    // Check if we reached goal
    if (next.first == 70 && next.second == 70) {
        break
    }

    // Add neighbors
    if ((0..70).contains(next.first + 1) && grid[next.first + 1][next.second].let { it.type == Block.Type.Empty && it.distance > grid[next.first][next.second].distance + 1 }) {
        grid[next.first + 1][next.second].seen = true
        grid[next.first + 1][next.second].distance = grid[next.first][next.second].distance + 1
        queue.addFirst(next.first + 1 to next.second)
    }
    if ((0..70).contains(next.second + 1) && grid[next.first][next.second + 1].let { it.type == Block.Type.Empty && it.distance > grid[next.first][next.second].distance + 1 }) {
        grid[next.first][next.second + 1].seen = true
        grid[next.first][next.second + 1].distance = grid[next.first][next.second].distance + 1
        queue.addFirst(next.first to next.second + 1)
    }
    if ((0..70).contains(next.first - 1) && grid[next.first - 1][next.second].let { it.type == Block.Type.Empty && it.distance > grid[next.first][next.second].distance + 1 }) {
        grid[next.first - 1][next.second].seen = true
        grid[next.first - 1][next.second].distance = grid[next.first][next.second].distance + 1
        queue.addFirst(next.first - 1 to next.second)
    }
    if ((0..70).contains(next.second - 1) && grid[next.first][next.second - 1].let { it.type == Block.Type.Empty && it.distance > grid[next.first][next.second].distance + 1 }) {
        grid[next.first][next.second - 1].seen = true
        grid[next.first][next.second - 1].distance = grid[next.first][next.second].distance + 1
        queue.addFirst(next.first to next.second - 1)
    }
}

println(grid[70][70].distance)