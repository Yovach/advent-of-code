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
  .split("\n")
  .filter((line) => line.length > 0);

const pageOrderingRules: PageOrderingRule[] = [];
const pageOrderingRulesCache: Map<string, boolean> = new Map();

function isUpdateBefore(before: number, after: number): boolean | undefined {
  const cachedKey = `${before}<${after}`;

  if (pageOrderingRulesCache.has(cachedKey)) {
    return pageOrderingRulesCache.get(cachedKey);
  }

  const value = false === pageOrderingRules.some((rule) =>
    rule.before === after && rule.after === before
  );
  pageOrderingRulesCache.set(cachedKey, value);

  return value;
}

function areUpdatesRightOrder(updates: number[]): boolean {
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

let total = 0;
for (const line of lines) {
  if (line.includes("|")) {
    // Page ordering rules
    const [before, after] = line.split("|").map(parseAsInt);
    pageOrderingRules.push({ before, after });
  } else if (line.includes(",")) {
    const updates = line.split(",").map(parseAsInt);
    const isValid = areUpdatesRightOrder(updates);
    if (isValid) {
      let middleNumberIndex = updates.length / 2;
      if (Number.isInteger(middleNumberIndex)) {
        middleNumberIndex -= 1;
      } else {
        middleNumberIndex = Math.floor(middleNumberIndex);
      }
      const middleNumber = updates.at(middleNumberIndex);
      assert(middleNumber);

      total += middleNumber;
    }
  }
}

console.log(total);
