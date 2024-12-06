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

function isUpdateBefore(before: number, after: number): boolean | undefined {
  if (
    pageOrderingRules.some((rule) =>
      rule.before === before && rule.after === after
    )
  ) {
    return true;
  }

  return undefined;
}

function areUpdatesRightOrder(updates: number[]) {
  console.log('analyse', updates)
  for (let idx = 0; idx < updates.length; idx++) {
    const update = updates.at(idx);
    assert(update);

    for (const nextValue of updates.slice(idx)) {
      if (false === isUpdateBefore(update, nextValue)) {
        return false;
      }
    }
  }
  
  return true;
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
    console.log({ updates, isValid: areUpdatesRightOrder(updates) });
  }
  console.log("\n NEXT \n")
}
