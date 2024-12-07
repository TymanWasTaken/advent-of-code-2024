#!/usr/bin/env kotlin

import java.io.File

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
    val iter = it.chars().iterator()
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



println("\n" + sum)