import { join as joinPath } from "@std/path";
import { assert } from "@std/assert";
import { getMapFromFile } from "./utils.ts";

assert(import.meta.dirname);

const filePath: string = joinPath(import.meta.dirname, "..", "input.txt");
const fileContent = Deno
  .readTextFileSync(filePath)
  .trimEnd();

const mapGrid: string[][] = getMapFromFile(fileContent);

console.log(mapGrid);
