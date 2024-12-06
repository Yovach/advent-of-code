import { join as joinPath } from "@std/path";
import { assert } from "@std/assert";

assert(import.meta.dirname);

const filePath = joinPath(import.meta.dirname, "..", "input.txt");
const lines: string[] = Deno
  .readTextFileSync(filePath)
  .trimEnd()
  .split("\n").filter((line) => line.length > 0);

for (const line of lines) {
  if (line.includes("|")) {
    // Page ordering rules
    console.log(line)
  } else if (line.includes(",")) {

  }
}
