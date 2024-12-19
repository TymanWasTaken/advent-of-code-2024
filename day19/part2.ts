const cache: Record<string, number> = {};

function attempt(towels: string[], pattern: string): number {
    if (pattern.length <= 0) return 1;
    if (pattern in cache) return cache[pattern];
    
    const val = towels
        .filter(t => pattern.startsWith(t))
        .map(t => attempt(towels, pattern.substring(t.length)))
        .reduce((acc, cur) => acc + cur, 0);
    cache[pattern] = val;
    return val;
}

export async function solve() {
    const [towels, patterns] = await Deno.readTextFile(
        import.meta.dirname + "/input.txt",
    ).then(
        (c) => [c.split("\n\n")[0].trim().split(", "), c.split("\n\n")[1].trim().split("\n")]
    );
    
    let sum = 0;
    for (const pattern of patterns) {
        sum += attempt(towels, pattern);
    }

    return sum
}
