import { assert } from "@std/assert";
import { join as joinPath } from "@std/path";
import {
  getGridFromFile,
  moveForward
} from "./utils.ts";

assert(import.meta.dirname);

const filePath: string = joinPath(import.meta.dirname, "..", "input.txt");
const fileContent = Deno
  .readTextFileSync(filePath)
  .trimEnd();

const { grid, player } = getGridFromFile(fileContent);
assert(player);

console.log(grid, player);

let nbDistinctPositions = 0;
let isStuck = false;
while (!isStuck) {
  isStuck = moveForward(grid, player);
}

console.log(nbDistinctPositions)
