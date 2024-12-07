import { assert } from "@std/assert";
import { join as joinPath } from "@std/path";
import { getInitialPlayerPosition, getMapFromFile, GridCell } from "./utils.ts";

assert(import.meta.dirname);

const filePath: string = joinPath(import.meta.dirname, "..", "input.txt");
const fileContent = Deno
  .readTextFileSync(filePath)
  .trimEnd();

const mapGrid: GridCell[] = getMapFromFile(fileContent);
const initialPosition = getInitialPlayerPosition(mapGrid);
console.log(mapGrid, initialPosition);

// let isStuck = false;
// while (!isStuck) {

// }
