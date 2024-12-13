#!/usr/bin/env kotlin

import java.io.File
import kotlin.math.floor

// I LOVE MATH
typealias Matrix2 = Pair<Pair<Int, Int>, Pair<Int, Int>>
fun Matrix2.determinant(): Int = (this.first.first * this.second.second) - (this.first.second * this.second.first)

typealias Point = Pair<Int, Int>

data class Game(
    val buttonA: Point,
    val buttonB: Point,
    val goal: Point
)

val games = File("input.txt").readLines().chunked(4).map { lines ->
    Game(
        buttonA = lines[0].substring(12).split(", Y+").let {
            it[0].toInt() to it[1].toInt()
        },
        buttonB = lines[1].substring(12).split(", Y+").let {
            it[0].toInt() to it[1].toInt()
        },
        goal = lines[2].substring(9).split(", Y=").let {
            it[0].toInt() to it[1].toInt()
        }
    )
}

var tokens = 0

games.forEach { game ->
    val aPresses = Matrix2(
        game.goal.first to game.buttonB.first,
        game.goal.second to game.buttonB.second
    ).determinant().toDouble() / Matrix2(
        game.buttonA.first to game.buttonB.first,
        game.buttonA.second to game.buttonB.second
    ).determinant()

    val bPresses = Matrix2(
        game.buttonA.first to game.goal.first,
        game.buttonA.second to game.goal.second
    ).determinant().toDouble() / Matrix2(
        game.buttonA.first to game.buttonB.first,
        game.buttonA.second to game.buttonB.second
    ).determinant()

    if (aPresses != floor(aPresses) || bPresses != floor(bPresses)) return@forEach

    tokens += aPresses.toInt() * 3 + bPresses.toInt()
}

println("Result: $tokens")