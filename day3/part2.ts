export async function solve() {
    const input = await Deno.readTextFile(import.meta.dirname + "/input.txt");
    let sum = 0;
    for (const part of input.split("do()").slice(1).map(e => e.split("don't()")[0])) {
        for (const match of part.matchAll(/mul\((\d{1,3}),(\d{1,3})\)/g)) {
            sum += +match[1] * +match[2];
        }
    }
    return sum
}