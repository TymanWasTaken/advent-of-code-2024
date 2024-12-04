export async function solve() {
    const grid = await Deno.readTextFile("./input.txt").then(i => i.split("\n").map(l => l.split("")));
    let sum = 0;

    for (const [y, line] of grid.map((e, i) => [i, e] as const)) {
        for (let x = 0; x < line.length; x++) {
            // Check horizontal forwards & backwards
            if (["XMAS", "SAMX"].includes(grid[y].slice(x, x + 4).join(""))) sum++;
            // Check vertical forwards & backwards
            if (["XMAS", "SAMX"].includes(grid.slice(y, y + 4).map(l => l[x]).join(""))) sum++;
            // Check diagonal \ forwards & backwards
            if (["XMAS", "SAMX"].includes(grid.slice(y, y + 4).map((l, i) => l[x + i]).join(""))) sum++;
            // Check diagonal / forwards & backwards
            if (["XMAS", "SAMX"].includes(grid.slice(y, y + 4).map((l, i) => l[x + (3 - i)]).join(""))) sum++;
        }
    }

    return sum
}
