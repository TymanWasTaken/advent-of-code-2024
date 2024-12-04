export async function solve() {
    const grid = await Deno.readTextFile(import.meta.dirname + "/input.txt").then(i => i.split("\n").map(l => l.split("")));
    let sum = 0;

    for (const [y, line] of grid.map((e, i) => [i, e] as const)) {
        for (let x = 0; x < line.length; x++) {
            if (
                // Check diagonal \ forwards & backwards
                ["MAS", "SAM"].includes(grid.slice(y, y + 3).map((l, i) => l[x + i]).join(""))
                // Check diagonal / forwards & backwards
                && ["MAS", "SAM"].includes(grid.slice(y, y + 3).map((l, i) => l[x + 2 - i]).join(""))
            ) sum++;
        }
    }

    return sum
}
