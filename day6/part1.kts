#!/usr/bin/env kotlin

import java.io.File

enum class Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT;

    fun rotate90(): Direction {
        return when (this) {
            UP -> RIGHT
            DOWN -> LEFT
            RIGHT -> DOWN
            LEFT -> UP
        }
    }
}

var grid = mutableListOf<MutableList<Char>>()

var guy = Pair(0, 0);
var direction = Direction.UP

var i = 0
File("./input.txt").forEachLine {
    var iter = it.chars().iterator()
    var j = 0
    grid.add(MutableList(it.length) {
        var next = iter.next().toChar()

        if (next == '^') {
            guy = Pair(i, j)
            next = '.'
        }

        j++
        next
    })

    i++
}

var steps = 0
while (true) {
    val next = when (direction) {
        Direction.UP -> Pair(guy.first - 1, guy.second)
        Direction.DOWN -> Pair(guy.first + 1, guy.second)
        Direction.RIGHT -> Pair(guy.first, guy.second + 1)
        Direction.LEFT -> Pair(guy.first, guy.second - 1)
    }


    try {
        if (grid[next.first][next.second] == '#') {
            direction = direction.rotate90()
        } else if (grid[next.first][next.second] == 'X') {
            guy = next
        } else {
            grid[next.first][next.second] = 'X'
            steps++
            guy = next
        }
    } catch (_: IndexOutOfBoundsException) {
        break
    }
}

println(steps)