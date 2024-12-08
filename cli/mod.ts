#!/usr/bin/env deno
import * as path from "@std/path";
import * as ini from "@std/ini";

import { Command } from "@cliffy/command";
import { HelpCommand } from "@cliffy/command/help";
import { Select } from "@cliffy/prompt";
import { dir, DirectoryTypes } from "@cross/dir";

import { Database } from "@db/sqlite";

type ProfileData = {
	Default: "0" | "1";
	IsRelative: "0" | "1";
	Name: string;
	Path: string;
};
type ProfilesIni = {
	General: {
		StartWithLastProfile: string;
		Verstion: string;
	};
} & Record<string, ProfileData>;

interface FirefoxContainer {
	color: string;
	icon: string;
	name: string;
	public: boolean;
	userContextId: number;
};
interface ContainersJson {
	identities: FirefoxContainer[];
	lastUserContextId: number;
	version: number;
}

if (import.meta.main) {
	await new Command()
		.name("aoc")
		.version("0.1.0")
		.description("My AoC utility command for 2024")
		.help({
			types: true,
			hints: true,
			colors: true
		})
		.default("help")
		.command("help", new HelpCommand().global())
		.command(
			"login",
			new Command()
				.description("Stores your advent of code session token")
				.default("help")
				.command(
					"firefox [profile:string] [container:string]",
					"Imports an advent of code session token from firefox"
				)
				.action(async (_: unknown, profileArg: string | undefined, containerArg: string | undefined) => {
					try {
						// Discover firefox profiles
						const firefoxProfilesPath = path.join(await dir(DirectoryTypes.home), ".mozilla", "firefox");

						const profiles =
							ini.parse(await Deno.readTextFile(path.join(firefoxProfilesPath, "profiles.ini"))) as ProfilesIni;

						const parsedProfiles = Object.entries(profiles).reduce((acc, [k, v]) => ({
							list: [
								...acc.list,
								...(
									k !== "General"
										? [{
											...(v as ProfileData),
											IsRelative: "0" as const,
											Path: path.resolve(
												path.join(...(
													(v as ProfileData).IsRelative == "0"
														? [(v as ProfileData).Path]
														: [firefoxProfilesPath, (v as ProfileData).Path]
												))
											)
										}]
										: []
								)
							],
							default: k !== "General" && (v as ProfileData).Default == "1"
								? acc.list.length
								: acc.default,
						}), { default: undefined, list: [] } as { default: number | undefined; list: ProfileData[] });

						// If the profile we need is ambigous, prompt the user for it
						let profile: ProfileData;

						if (profileArg !== undefined && parsedProfiles.list.find(d => d.Name == profileArg) !== undefined) {
							profile = parsedProfiles.list.find(d => d.Name == profileArg)!;
						} else {
							let message: string
							if (profileArg !== undefined) {
								message = "Unable to find the specified profile name, please select the correct one";
							} else {
								message = "Please select the correct profile to import session token from";
							}

							const options = parsedProfiles.list.map(p =>
								[
									`${p.Name} (${path.resolve(
										path.join(...(
											p.IsRelative == "0"
												? [p.Path]
												: [firefoxProfilesPath, p.Path]
										))
									)})`,
									p
								] as const
							);

							const selection = await Select.prompt({
								message,
								default: parsedProfiles.default !== undefined ? options[parsedProfiles.default][0] : undefined,
								options: options.map(([e,]) => e)
							});

							profile = options.find(([e,]) => e == selection)![1];
						}

						// If necessary, differentiate between containers
						let container: FirefoxContainer | null = null;
						const containerFile = await Deno.readTextFile(
							path.join(profile.Path, "containers.json")
						).then(t => JSON.parse(t) as ContainersJson).catch(_ => null);

						if (containerFile === null && containerArg) {
							console.error("Container was specified, but the selected profile has no containers");
							Deno.exit(1);
						} else if (containerFile !== null) {
							const matchingContainer = containerArg === undefined
								? undefined
								: containerFile.identities.find(
									i => i.name.toLowerCase() == containerArg.toLowerCase()
								);
							
							if (matchingContainer === undefined) {
								const message = containerArg === undefined
									? "Please select the container to import cookies from"
									: "The specified container could not be found, please";
								
								const choice = await Select.prompt({
									message,
									options: containerFile.identities.filter(c => c.public).map(c => c.name),
									default: containerFile.identities.find(c => c.userContextId == containerFile.lastUserContextId)!.name
								});

								container = containerFile.identities.find(c => c.name == choice)!;
							} else {
								container = matchingContainer;
							}
						}

						// Import the cookies
						const tmpFile = await Deno.makeTempFile({ prefix: "aoc-firefox-cookies" });
						await Deno.copyFile(path.join(profile.Path, "cookies.sqlite"), tmpFile);
						const cookies = new Database(tmpFile, {
							create: false,
							readonly: true
						});
						let rows: { value: string }[];
						try {
							rows = container === null
								? cookies.sql`
									SELECT value
									FROM moz_cookies
									WHERE name='session' AND host='.adventofcode.com';
								`
								: cookies.sql`
									SELECT value
									FROM moz_cookies
									WHERE (
										originAttributes LIKE ${`%userContextId=${container.userContextId}`}
										OR originAttributes LIKE ${`%userContextId=${container.userContextId}&%`}
									) AND name='session' AND host='.adventofcode.com';
								`;
						} catch (_) {
							rows = [];
						}
						
						if (rows.length < 1) {
							console.error("No advent of code session cookie was found, make sure you are logged in!");
							Deno.exit(1);
						} else if (rows.length > 1) {
							console.error("Multiple advent of code session cookies were found??");
							Deno.exit(1);
						} else {
							const process = new Deno.Command("secret-tool", {
								args: [
									"store",
									"--label",
									"Advent Of Code Session Token",
									"token",
									"value"
								],
								stdin: "piped"
							}).spawn();
							const stdin = process.stdin.getWriter();
							await stdin.write(new TextEncoder().encode(rows[0].value));
							await stdin.close();
							console.log("Token saved!");
						}

						await Deno.remove(tmpFile);
					} catch (e) {
						console.log(e)
					}
				})
		)
		.reset()
		.command(
			"input [day:number]",
			"Fetches the input for one day (or all if day # omitted) and writes to input.txt files"
		)
		.action((_: unknown, day: number | undefined) => {
			if (day != undefined) {

			}
		})
		.parse(Deno.args);
}
