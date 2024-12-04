export async function solve() {
    const [rules, pages] = await Deno.readTextFile("./input.txt")
        .then(i => i
            .split("\n\n")
        ).then(([rules, pages]) => [
            rules.split("\n").map(e => e.split("|") as [string, string]),
            pages.split("\n").map(e => e.split(","))
        ] as const);
    
    let sum = 0;

    for (const pageGroup of pages) {
        let valid = true;
        for (let i = 1; i < pageGroup.length; i++) {
            const matchingRules = rules.filter(([before, _]) => before == pageGroup[i]).filter(
                ([before, _]) => before == pageGroup[i]
            ).map(([_, after]) => after);
            if (matchingRules.some(after => pageGroup.slice(0, i).includes(after))) {
                valid = false;
                break
            }
        }
        if (valid) sum += +pageGroup[Math.floor(pageGroup.length / 2)];
    }

    return sum
}
