export async function solve() {
    return await Deno.readTextFile(import.meta.dirname + "/input.txt").then(
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
    ).then(([first, second]) => [
        first,
        second.reduce((acc, cur) => {
            if (acc[cur] !== undefined) acc[cur]++;
            else acc[cur] = 1;

            return acc;
        }, {} as Record<number, number>),
    ] as const).then(([first, map]) =>
        first.reduce((acc, cur) => acc + ((map[cur] ?? 0) * cur), 0)
    );
}
