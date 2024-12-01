export async function solve() {
    return await Deno.readTextFile("./input.txt").then(
        (f) =>
            f.split("\n").reduce<[number[], number[]]>(
                (
                    acc,
                    cur,
                ) => [
                    [...acc[0], +cur.split("   ")[0]],
                    [...acc[1], +cur.split("   ")[1]],
                ],
                [[], []],
            ),
    ).then(([first, second]) => [first.toSorted(), second.toSorted()]).then(
        ([first, second]) =>
            first.reduce((acc, cur, i) => acc + Math.abs(cur - second[i]), 0),
    );
}
