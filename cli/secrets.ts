export async function saveToken(token: string) {
    const process = new Deno.Command("secret-tool", {
        args: [
            "store",
            "--label",
            "Advent Of Code Session Token",
            "aoc-deno",
            "token"
        ],
        stdin: "piped"
    }).spawn();
    const stdin = process.stdin.getWriter();
    await stdin.write(new TextEncoder().encode(token));
    await stdin.close();
}

export async function readToken() {
    const process = new Deno.Command("secret-tool", {
        args: [
            "lookup",
            "aoc-deno",
            "token"
        ],
        stdout: "piped"
    }).spawn();
    const decoderStream = new TextDecoderStream();
    const stdout = process.stdout.pipeTo(decoderStream.writable);
    for await (const v of decoderStream.readable.values()) {
        console.log(v);
    }
}