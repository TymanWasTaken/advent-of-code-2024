export async function solve() {
    return await Deno.readTextFile("./input.txt").then((f) =>
        f.split("\n").filter((l) => {
            const report = l.split(" ").map((s) => +s);
            for (let i = 0; i < report.length; i++) {
                const sliced = [
                    ...report.slice(0, i),
                    ...report.slice(i + 1, report.length),
                ];
                const increasing = sliced[0] < sliced[1];
                let last = sliced[0];
                let failed = false;
                for (const number of sliced.slice(1)) {
                    if (number > last && !increasing) failed = true;
                    if (number < last && increasing) failed = true;
                    if (
                        Math.abs(number - last) < 1 ||
                        Math.abs(number - last) > 3
                    ) failed = true;

                    last = number;
                }
                if (failed) continue;
                else return true;
            }
            return false;
        }).reduce((acc) => acc + 1, 0)
    );
}
