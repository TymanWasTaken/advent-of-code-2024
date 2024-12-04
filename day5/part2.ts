function sort(rules: [string, string][], pageGroup: string[]) {
    let [valid, error] = validate(rules, pageGroup);
    while (!valid) {
        const tmp = pageGroup[error![0]];
        pageGroup[error![0]] = pageGroup[error![1]];
        pageGroup[error![1]] = tmp;

        [valid, error] = validate(rules, pageGroup);
    }
}

function validate(rules: [string, string][], pageGroup: string[]): [boolean, null | [number, number]] {
    for (let i = 1; i < pageGroup.length; i++) {
        const matchingRules = rules.filter(([before, _]) => before == pageGroup[i]).map(([_, after]) => after);
        const error = matchingRules.find(after => pageGroup.slice(0, i).includes(after));
        if (error !== undefined) {
            return [false, [pageGroup.indexOf(error), i]];
        }
    }

    return [true, null]
}

export async function solve() {
    const [rules, pages] = await Deno.readTextFile("./input.txt")
        .then(i => i
            .split("\n\n")
        ).then(([rules, pages]) => [
            rules.split("\n").map(e => e.split("|") as [string, string]),
            pages.split("\n").map(e => e.split(","))
        ] as const);
    
    let sum = 0;

    for (let pageGroup of pages) {
        let [valid,error] = validate(rules, pageGroup);
        if (valid !== true) {
            // console.log(pageGroup, error);
            sort(rules, pageGroup);

            sum += +pageGroup[Math.floor(pageGroup.length / 2)];
        };
    }

    return sum
}
