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
        // Horizontal R-L
        getLetter(horizontalIdx - 1, verticalIdx) === "M" &&
        getLetter(horizontalIdx - 2, verticalIdx) === "A" &&
        getLetter(horizontalIdx - 3, verticalIdx) === "S"
      ) {
        nbOccurrences++;
      }

      if (
        // Horizontal L-R
        getLetter(horizontalIdx + 1, verticalIdx) === "M" &&
        getLetter(horizontalIdx + 2, verticalIdx) === "A" &&
        getLetter(horizontalIdx + 3, verticalIdx) === "S"
      ) {
        nbOccurrences++;
      }

      if (
        // Vertical B-T
        getLetter(horizontalIdx, verticalIdx - 1) === "M" &&
        getLetter(horizontalIdx, verticalIdx - 2) === "A" &&
        getLetter(horizontalIdx, verticalIdx - 3) === "S"
      ) {
        nbOccurrences++;
      }

      if (
        // Vertical T-B
        getLetter(horizontalIdx, verticalIdx + 1) === "M" &&
        getLetter(horizontalIdx, verticalIdx + 2) === "A" &&
        getLetter(horizontalIdx, verticalIdx + 3) === "S"
      ) {
        nbOccurrences++;
      }

      if (
        // Diagonal TL-BR
        getLetter(horizontalIdx + 1, verticalIdx + 1) === "M" &&
        getLetter(horizontalIdx + 2, verticalIdx + 2) === "A" &&
        getLetter(horizontalIdx + 3, verticalIdx + 3) === "S"
      ) {
        nbOccurrences++;
      }


      if (
        // Diagonal BR-TL
        getLetter(horizontalIdx - 1, verticalIdx - 1) === "M" &&
        getLetter(horizontalIdx - 2, verticalIdx - 2) === "A" &&
        getLetter(horizontalIdx - 3, verticalIdx - 3) === "S"
      ) {
        nbOccurrences++;
      }

      if (
        // Diagonal BL-TR
        getLetter(horizontalIdx + 1, verticalIdx - 1) === "M" &&
        getLetter(horizontalIdx + 2, verticalIdx - 2) === "A" &&
        getLetter(horizontalIdx + 3, verticalIdx - 3) === "S"
      ) {
        nbOccurrences++;
      }

      if (
        // Diagonal TR-BL
        getLetter(horizontalIdx - 1, verticalIdx + 1) === "M" &&
        getLetter(horizontalIdx - 2, verticalIdx + 2) === "A" &&
        getLetter(horizontalIdx - 3, verticalIdx + 3) === "S"
      ) {
        nbOccurrences++;
      }
    }
  }
}

console.log({ nbOccurrences });
