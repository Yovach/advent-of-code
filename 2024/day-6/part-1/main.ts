import { assert } from "@std/assert";
import { join as joinPath } from "@std/path";
import { getGridFromFile, moveForwardUntilStuck } from "./utils.ts";

assert(import.meta.dirname);

const filePath: string = joinPath(import.meta.dirname, "..", "input.txt");
const fileContent = Deno
  .readTextFileSync(filePath)
  .trimEnd();

const { grid, player } = getGridFromFile(fileContent);
assert(player);

const visitedDistricts = moveForwardUntilStuck(grid, player);
console.log(visitedDistricts.length);
