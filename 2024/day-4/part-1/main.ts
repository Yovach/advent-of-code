import { join as joinPath } from "@std/path";
import { assert } from "@std/assert";

assert(import.meta.dirname);

const filePath = joinPath(import.meta.dirname, "..", "input.txt");
const fileContent: string = Deno.readTextFileSync(filePath).trimEnd();

const directions = [-1, 0, 1];
let nbOccurrences = 0;

const lines = fileContent.split("\n");
for (
  let verticalIdx = 0;
  verticalIdx < lines.length;
  verticalIdx++
) {
  const line = lines[verticalIdx];

  const chars = line.split("");
  for (
    let horizontalIdx = 0;
    horizontalIdx < chars.length;
    horizontalIdx++
  ) {
    const character = chars[horizontalIdx];

    if (character === "X") {
      if (
        chars[horizontalIdx - 1] === "M" &&
        chars[horizontalIdx - 2] === "A" &&
        chars[horizontalIdx - 3] === "S"
      ) {
        nbOccurrences++;
      } else if (
        chars[horizontalIdx + 1] === "M" &&
        chars[horizontalIdx + 2] === "A" &&
        chars[horizontalIdx + 3] === "S"
      ) {
        nbOccurrences++;
      }
    }
  }
}

console.log({ nbOccurrences });
