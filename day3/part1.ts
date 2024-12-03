export async function solve() {
    const input = await Deno.readTextFile("./input.txt");
    let sum = 0;
    for (const match of input.matchAll(/mul\((\d{1,3}),(\d{1,3})\)/g)) {
        sum += +match[1] * +match[2];
    }
    return sum
}