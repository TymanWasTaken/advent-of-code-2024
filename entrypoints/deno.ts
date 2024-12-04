import * as path from "@std/path"

// Parse all modules
const parentDir = path.resolve(path.join(import.meta.dirname!, ".."));
const dayFolderRegex = /^day(\d{1,2})$/;
const days: { day: number, part: 1|2, solve: () => Promise<number> }[] = [];
for await (const folder of Deno.readDir(parentDir)) {
    const regex = folder.name.match(dayFolderRegex);
    if (!regex) {
        continue;
    }
    const dayNum = regex[1];

    for (let i = 1; i <= 2; i++) {
        if (await Deno.stat(path.join(parentDir, "day" + dayNum, `part${i}.ts`)).then(s => s.isFile).catch(_ => false)) {
            const module = await import(path.join(parentDir, "day" + dayNum, `part${i}.ts`));
            days.push({
                day: +dayNum,
                part: i as 1|2,
                solve: module.solve
            })
        }
    }
}

// Create deno benchmark entries
for (const { day, part, solve } of days) {
    Deno.bench(
        `Day ${day} Part ${part}`,
        // Group each day and say how much faster p1 is than p2
        { group: `Day ${day}`, baseline: part == 1, permissions: { "read": true } },
        () => solve().then(_ => undefined)
    )
}