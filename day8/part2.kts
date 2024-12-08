#!/usr/bin/env kotlin

import java.io.File

val frequencies = mutableMapOf<Char, MutableList<Pair<Int, Int>>>()
var gridWidth = 0
var gridHeight = 0
File("./input.txt").readLines().also {
    gridHeight = it.size
}.forEachIndexed { i, line ->
    line.also { gridWidth = it.length }.forEachIndexed chars@{ j, freq ->
        if (freq == '.') return@chars
        val curValue = frequencies.putIfAbsent(
            freq, mutableListOf(Pair(i, j))
        )
        if (curValue !== null) {
            curValue.add(Pair(i, j))
        }
    }
}

val antiNodes = hashSetOf<Pair<Int, Int>>()
frequencies.forEach { (_, points) ->
    points.forEach outer@{ point1 ->
        points.forEach { point2 ->
            if (point1 == point2) return@outer
            val dRow = (point1.first - point2.first)
            val dCol = (point1.second - point2.second)

            var curRow = point1.first
            var curCol = point1.second
            while (curRow in 0..<gridHeight && curCol in 0..<gridWidth) {
                antiNodes.add(Pair(curRow, curCol))
                curRow += dRow
                curCol += dCol
            }

            curRow = point2.first
            curCol = point2.second
            while (curRow in 0..<gridHeight && curCol in 0..<gridWidth) {
                antiNodes.add(Pair(curRow, curCol))
                curRow -= dRow
                curCol -= dCol
            }
        }
    }
}

println(antiNodes.size)
