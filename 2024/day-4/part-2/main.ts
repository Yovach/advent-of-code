import { join as joinPath } from "@std/path";
import { assert } from "@std/assert";

assert(import.meta.dirname);

const filePath = joinPath(import.meta.dirname, "..", "input.txt");
const fileContent: string = Deno.readTextFileSync(filePath)
  .trimEnd()
  .replaceAll("X", ".");

const lines = fileContent.split("\n");

type Coordinate = [x: number, y: number];

const directions: Coordinate[] = [[-1, -1], [-1, 1], [1, -1], [1, 1]];

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
      let foundXMas = 0;

      for (let idx = 0; idx < directions.length; idx++) {
        const [x, y] = directions[idx];
        const letter = getLetter(horizontalIdx + x, verticalIdx + y);
        if (letter === "M") {
          const oppositeCoordinates = directions.at(
            (directions.length - 1) - idx,
          );
          if (oppositeCoordinates === undefined) {
            continue;
          }

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

      if (foundXMas === 2 ) {
        nbOccurrences++;
      }
    }
  }
}

console.log({ nbOccurrences });
Deno.writeTextFile("./output.txt", lines.join("\n"));
