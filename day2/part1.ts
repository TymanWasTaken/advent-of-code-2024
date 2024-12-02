export async function solve() {
    return await Deno.readTextFile("./input.txt").then((f) =>
        f.split("\n").filter((l) => {
            const report = l.split(" ").map((s) => +s);
            const increasing = report[0] < report[1];
            let last = report[0];
            for (const number of report.slice(1)) {
                if (number > last && !increasing) return false;
                if (number < last && increasing) return false;
                if (Math.abs(number - last) < 1 || Math.abs(number - last) > 3) return false;
    
                last = number;
            }
            return true;
        }).reduce(acc => acc + 1, 0)
    );    
}