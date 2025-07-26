import { argv } from "node:process";
import { readFile, access } from "fs/promises";
import { constants } from "fs";
import { Result, ok, err } from "neverthrow";

type CliFlag =
  | { type: "Chars"; count: (b: Buffer) => number }
  | { type: "Lines"; count: (b: Buffer) => number }
  | { type: "Words"; count: (b: Buffer) => number }
  | { type: "Locale"; count: (b: Buffer) => number };

const CliFlag = {
  Chars: (): CliFlag => ({
    type: "Chars",
    count: (b: Buffer) => b.length,
  }),
  Lines: (): CliFlag => ({
    type: "Lines",
    count: (b: Buffer) => {
      const s = b.toString("utf-8");
      return s.split("\n").length;
    },
  }),
  Words: (): CliFlag => ({
    type: "Words",
    count: (b: Buffer) => {
      const s = b.toString("utf-8");
      return s.split(/\s+/).filter((word) => word.length > 0).length;
    },
  }),
  Locale: (): CliFlag => ({
    type: "Locale",
    count: (b: Buffer) => {
      const locale = detectEncodingHeuristic(b);
      const s = b.toString(locale ? locale : undefined);
      return s.length;
    },
  }),
};

const USAGE = "USAGE: wc -[lmwc] file";

function parseArguments(args: string[]): Result<[string, CliFlag], string> {
  if (args.length < 4) {
    return err(USAGE);
  }
  let cliFlag: CliFlag;
  switch (args[2]) {
    case "-c":
      cliFlag = CliFlag.Chars();
      break;
    case "-l":
      cliFlag = CliFlag.Lines();
      break;
    case "-w":
      cliFlag = CliFlag.Words();
      break;
    case "-m":
      cliFlag = CliFlag.Locale();
      break;
    default:
      return err(USAGE);
  }
  return ok([args[3], cliFlag]);
}

async function read(path: string): Promise<Result<Buffer, string>> {
  try {
    await access(path, constants.F_OK | constants.R_OK);
    const data = await readFile(path);
    return ok(data);
  } catch (error: any) {
    if (error.code === "ENOENT") {
      return err(`${path} does not exist`);
    }
    if (error.code === "EACCES") {
      return err(`Permission denied: ${path}`);
    }
    return err(error.message || String(error));
  }
}

async function main() {
  const argsResult = parseArguments(argv);
  if (argsResult.isErr()) {
    console.log(argsResult.mapErr((err) => err));
    return;
  }
  const [path, cliFlag] = argsResult._unsafeUnwrap();
  const buffer = (await read(path))._unsafeUnwrap();
  console.log(`${cliFlag.count(buffer)} ${path}`);
}

await main();

function detectEncodingHeuristic(buffer: Buffer): NodeJS.BufferEncoding | null {
  if (isValidUTF8(buffer)) {
    return "utf-8";
  }
  if (isASCII(buffer)) {
    return "ascii";
  }

  return null;
}

function isValidUTF8(buffer: Buffer): boolean {
  try {
    const str = buffer.toString("utf8");
    return Buffer.from(str, "utf8").equals(buffer);
  } catch {
    return false;
  }
}

function isASCII(buffer: Buffer): boolean {
  for (let i = 0; i < buffer.length; i++) {
    if (buffer[i] > 127) {
      return false;
    }
  }
  return true;
}
