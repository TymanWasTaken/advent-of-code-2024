#!/usr/bin/env kotlin

import java.io.File
import kotlin.math.floor

// I LOVE MATH
typealias Matrix2 = Pair<Pair<Long, Long>, Pair<Long, Long>>
fun Matrix2.determinant(): Long = (this.first.first * this.second.second) - (this.first.second * this.second.first)

typealias Point = Pair<Long, Long>

data class Game(
    val buttonA: Point,
    val buttonB: Point,
    val goal: Point
)

val games = File("input.txt").readLines().chunked(4).map { lines ->
    Game(
        buttonA = lines[0].substring(12).split(", Y+").let {
            it[0].toLong() to it[1].toLong()
        },
        buttonB = lines[1].substring(12).split(", Y+").let {
            it[0].toLong() to it[1].toLong()
        },
        goal = lines[2].substring(9).split(", Y=").let {
            it[0].toLong() + 10000000000000 to it[1].toLong() + 10000000000000
        }
    )
}

var tokens = 0L

games.forEach { game ->
    val aPressesNumer = Matrix2(
        game.goal.first to game.buttonB.first,
        game.goal.second to game.buttonB.second
    ).determinant()
    val aPressesDenom = Matrix2(
        game.buttonA.first to game.buttonB.first,
        game.buttonA.second to game.buttonB.second
    ).determinant()

    val bPressesNumer = Matrix2(
        game.buttonA.first to game.goal.first,
        game.buttonA.second to game.goal.second
    ).determinant()
    val bPressesDenom = Matrix2(
        game.buttonA.first to game.buttonB.first,
        game.buttonA.second to game.buttonB.second
    ).determinant()

    if (aPressesNumer % aPressesDenom == 0L && bPressesNumer % bPressesDenom == 0L) {
        tokens += (aPressesNumer / aPressesDenom) * 3 + (bPressesNumer / bPressesDenom)
    }
}

println("Result: $tokens")