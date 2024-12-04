import { join as joinPath } from "@std/path";
import { assert } from "@std/assert";

assert(import.meta.dirname);

const filePath = joinPath(import.meta.dirname, "..", "input.txt");
const fileContent: string = Deno.readTextFileSync(filePath).trimEnd();
const lines = fileContent.split("\n");

type Coordinate = [x: number, y: number];

const directions: Coordinate[] = [[-1, -1], [-1, 1], [1, 1], [1, -1]];

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

    if (character === "A") {

      const nbM = [];
      for (let idx = 0; idx < directions.length; idx++) {
        const [x, y] = directions[idx];
        const letter = getLetter(horizontalIdx + x, verticalIdx + y);
        if (letter === "M") {
          nbM.push({
            x, y
          })
        }

        const [oppositeX, oppositeY] = directions[(directions.length - 1) - idx];
        const oppositeLetter = getLetter(horizontalIdx + oppositeX, verticalIdx + oppositeY);
        if (oppositeLetter === "S") {
          nbM.push({
            x, y
          })
        }
      }

      if (nbM.length === 2 || nbM.length === 4) {
        nbOccurrences++;
      }
    }
  }
}

console.log({ nbOccurrences });
