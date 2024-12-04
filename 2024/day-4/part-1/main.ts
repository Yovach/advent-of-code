import { join as joinPath } from "@std/path";
import { assert } from "@std/assert";

assert(import.meta.dirname);

const filePath = joinPath(import.meta.dirname, "..", "input.txt");
const fileContent: string = Deno.readTextFileSync(filePath).trimEnd();

const directions = [-1, 0, 1];
let nbOccurrences = 0;

const lines = fileContent.split("\n");
for (
  let verticalIdx = 0, nbLines = lines.length;
  verticalIdx < nbLines;
  verticalIdx++
) {
  const line = lines[verticalIdx];

  const characters = line.split("");

  for (
    let horizontalIdx = 0, nbChars = characters.length;
    horizontalIdx < nbChars;
    horizontalIdx++
  ) {
    const character = characters[horizontalIdx];

    if (character === "X") {
      if (
        characters[horizontalIdx - 1] === "M" &&
        characters[horizontalIdx - 2] === "A" &&
        characters[horizontalIdx - 3] === "S"
      ) {
        nbOccurrences++;
      } else if (
        characters[horizontalIdx + 1] === "M" &&
        characters[horizontalIdx + 2] === "A" &&
        characters[horizontalIdx + 3] === "S"
      ) {
        nbOccurrences++;
      }
    }
  }
}

console.log({ nbOccurrences });
