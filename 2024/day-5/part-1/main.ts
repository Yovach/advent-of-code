import { join as joinPath } from "@std/path";
import { assert } from "@std/assert";

function parseAsInt(val: string) {
  return parseInt(val, 10);
}

assert(import.meta.dirname);

type PageOrderingRule = {
  before: number;
  after: number;
};

const filePath = joinPath(import.meta.dirname, "..", "input.txt");
const lines: string[] = Deno
  .readTextFileSync(filePath)
  .trimEnd()
  .split("\n").filter((line) => line.length > 0);

const pageOrderingRules: PageOrderingRule[] = [];

function areUpdatesRightOrder(updates: number[]) {
  for (const update of updates) {
  }
}

for (const line of lines) {
  if (line.includes("|")) {
    // Page ordering rules
    console.log(line);
    const [before, after] = line.split("|").map(parseAsInt);
    pageOrderingRules.push({ before, after });

    console.log(`before: ${before}, after: ${after}`);
  } else if (line.includes(",")) {
    const updates = line.split(",").map(parseAsInt);
    console.log({ updates });
  }
}
