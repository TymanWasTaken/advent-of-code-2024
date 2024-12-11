/**
 * There is no proper OS-secrets library for deno (or even nodejs!) so this is a
 * jank way to get around that on linux at least
 */
import * as io from "@std/io";

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
    const { stdout } = await new Deno.Command("secret-tool", {
        args: [
            "lookup",
            "aoc-deno",
            "token"
        ],
        stdout: "piped"
    }).spawn().output();
    return new TextDecoder().decode(stdout);
}