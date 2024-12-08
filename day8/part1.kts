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

            val antiNode1 = Pair(point1.first + dRow, point1.second + dCol)
            val antiNode2 = Pair(point2.first - dRow, point2.second - dCol)

            if (
                antiNode1.first >= 0
                && antiNode1.second >= 0
                && antiNode1.first < gridHeight
                && antiNode1.second < gridWidth) {
                antiNodes.add(antiNode1)
            }

            if (
                antiNode2.first >= 0
                && antiNode2.second >= 0
                && antiNode2.first < gridHeight
                && antiNode2.second < gridWidth) {
                antiNodes.add(antiNode2)
            }
        }
    }
}

println(antiNodes.size)
