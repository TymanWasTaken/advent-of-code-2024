#!/usr/bin/env kotlin

import java.io.File

object Constants {
    const val CONSECUTIVE_REPEATS_THRESHOLD = 200
}

fun MutableList<MutableList<Char>>.clone(): MutableList<MutableList<Char>> {
    val grid = mutableListOf<MutableList<Char>>()
    var i = 0;
    this.forEach {
        grid.add(MutableList(it.size) { j ->
            this[i][j]
        })
        i++
    }

    return grid
}

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

var originalGrid = mutableListOf<MutableList<Char>>()

var originalGuy = Pair(0, 0);
var originalDirection = Direction.UP

var i = 0
File("./input.txt").forEachLine {
    var iter = it.chars().iterator()
    var j = 0
    originalGrid.add(MutableList(it.length) {
        var next = iter.next().toChar()

        if (next == '^') {
            originalGuy = Pair(i, j)
            next = '.'
        }

        j++
        next
    })

    i++
}

var sum = 0;
originalGrid.forEachIndexed rowLoop@{ i, row ->
    row.forEachIndexed colLoop@{ j, _ ->
        if (originalGrid[i][j] != '.') return@colLoop
        val grid = originalGrid.clone()
        grid[i][j] = '#'

        var guy = originalGuy
        var direction = originalDirection
        var consecutiveRepeats = 0

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
                    consecutiveRepeats++
                    guy = next
                } else {
                    grid[next.first][next.second] = 'X'
                    consecutiveRepeats = 0
                    guy = next
                }
            } catch (_: IndexOutOfBoundsException) {
                break
            }
            if (consecutiveRepeats >= Constants.CONSECUTIVE_REPEATS_THRESHOLD) {
                sum++
                break
            }
        }

        print(
            // Clear line
            "\r                                          " +
                    // Print status
                    "\rChecked row ${
                        i.toString().padStart(3, ' ')
                    } column ${j.toString().padStart(3, ' ')} (${
                        (((i * row.size + j + 1) * 100) / (originalGrid.size * originalGrid[0].size)).toString()
                            .padStart(3, ' ')
                    }%)"
        )
    }
}

println("\n" + sum)