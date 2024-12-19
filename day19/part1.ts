function attempt(towels: string[], pattern: string): boolean {
    if (pattern == "") return true;
    let anySucceeded = false;
    for (const towel of towels) {
        if (pattern.startsWith(towel)) {
            anySucceeded = anySucceeded || attempt(towels, pattern.substring(towel.length));
        }
    }
    return anySucceeded;
}

export async function solve() {
    const [towels, patterns] = await Deno.readTextFile(
        import.meta.dirname + "/input.txt",
    ).then(
        (c) => [c.split("\n\n")[0].trim().split(", "), c.split("\n\n")[1].trim().split("\n")]
    );
    towels.sort((a, b) => b.length - a.length);
    
    let sum = 0;
    for (const pattern of patterns) {
        if (attempt(towels, pattern)) sum++;
    }

    return sum
}
