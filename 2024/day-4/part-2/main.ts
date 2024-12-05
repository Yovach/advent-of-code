import { join as joinPath } from "@std/path";
import { assert } from "@std/assert";

assert(import.meta.dirname);

type Coordinate = [x: number, y: number];
const DIRECTIONS: Coordinate[] = [[-1, -1], [-1, 1], [1, -1], [1, 1]];

const filePath = joinPath(import.meta.dirname, "..", "input.txt");
const lines: string[] = Deno
  .readTextFileSync(filePath)
  .trimEnd()
  .split("\n");

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
  for (
    let horizontalIdx = 0;
    horizontalIdx < line.length;
    horizontalIdx++
  ) {
    if (line[horizontalIdx] === "A") {
      let foundXMas = 0;

      for (let idx = 0; idx < DIRECTIONS.length; idx++) {
        const [x, y] = DIRECTIONS[idx];

        const letter = getLetter(horizontalIdx + x, verticalIdx + y);
        if (letter === "M") {
          // Si on ne trouve pas la direction, le programme doit crash
          const oppositeCoordinates = DIRECTIONS.at(
            (DIRECTIONS.length - 1) - idx,
          );
          assert(oppositeCoordinates);

          const [oppositeX, oppositeY] = oppositeCoordinates;
          const oppositeLetter = getLetter(
            horizontalIdx + oppositeX,
            verticalIdx + oppositeY,
          );
          if (oppositeLetter === "S") {
            foundXMas++;
          }
        }
      }

      if (foundXMas === 2) {
        nbOccurrences++;
      }
    }
  }
}

console.log(nbOccurrences);
