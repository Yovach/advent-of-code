import { join as joinPath } from "@std/path";
import { assert } from "@std/assert";

assert(import.meta.dirname);

const filePath = joinPath(import.meta.dirname, "..", "input.txt");
const fileContent: string = Deno.readTextFileSync(filePath).trimEnd();
const lines = fileContent.split("\n");

function getLetter(x: number, y: number): string | undefined {
  return lines[y]?.[x];
}

let nbOccurrences = 0;
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
        // Horizontal
        (
          chars[horizontalIdx - 1] === "M" &&
          chars[horizontalIdx - 2] === "A" &&
          chars[horizontalIdx - 3] === "S"
        ) || (
          chars[horizontalIdx + 1] === "M" &&
          chars[horizontalIdx + 2] === "A" &&
          chars[horizontalIdx + 3] === "S"
        )
      ) {
        console.log('horitonal')
        nbOccurrences++;
      }

      if (
        // Vertical
        (
          lines[verticalIdx - 1]?.[horizontalIdx] === "M" &&
          lines[verticalIdx - 2]?.[horizontalIdx] === "A" &&
          lines[verticalIdx - 3]?.[horizontalIdx] === "S"
        ) || (
          lines[verticalIdx + 1]?.[horizontalIdx] === "M" &&
          lines[verticalIdx + 2]?.[horizontalIdx] === "A" &&
          lines[verticalIdx + 3]?.[horizontalIdx] === "S"
        )
      ) {
        console.log("vertical")
        nbOccurrences++;
      }

      if (
        // Diagonal \
        (
          lines[verticalIdx - 1]?.[horizontalIdx - 1] === "M" &&
          lines[verticalIdx - 2]?.[horizontalIdx - 2] === "A" &&
          lines[verticalIdx - 3]?.[horizontalIdx - 3] === "S"
        ) || (
          lines[verticalIdx + 1]?.[horizontalIdx + 1] === "M" &&
          lines[verticalIdx + 2]?.[horizontalIdx + 2] === "A" &&
          lines[verticalIdx + 3]?.[horizontalIdx + 3] === "S"
        )
      ) {
        console.log("diagonal \\")
        nbOccurrences++;
      }

      if (
        // Diagonal /
        (
          lines[verticalIdx - 1]?.[horizontalIdx + 1] === "M" &&
          lines[verticalIdx - 2]?.[horizontalIdx + 2] === "A" &&
          lines[verticalIdx - 3]?.[horizontalIdx + 3] === "S"
        ) || (
          lines[verticalIdx + 1]?.[horizontalIdx - 1] === "M" &&
          lines[verticalIdx + 2]?.[horizontalIdx - 2] === "A" &&
          lines[verticalIdx + 3]?.[horizontalIdx - 3] === "S"
        )
      ) {
        console.log("diagonal /")
        nbOccurrences++;
      }
    }
  }
}

console.log({ nbOccurrences });
