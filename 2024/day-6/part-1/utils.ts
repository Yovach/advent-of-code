import { assert } from "@std/assert";

export enum CellType {
  EMPTY = "EMPTY",
  OBSTACLE = "OBSTACLE",
  PLAYER = "PLAYER",
}

type GridCell = {
  x: number;
  y: number;
  type: CellType;
};

const cellTypeMapping = Object.freeze({
  ".": CellType.EMPTY,
  "#": CellType.OBSTACLE,
});

export function getMapFromFile(fileContent: string): GridCell[] {
  const mapGrid: GridCell[] = [];
  const lines = fileContent.split("\n");
  console.log(lines);
  for (let y = 0; y < lines.length; y++) {
    const line = lines.at(y);
    assert(line);

    for (let x = 0; x < line.length; x++) {
      const char = line.at(x);
      assert(char);

      let type: CellType;
      if (Object.hasOwn(cellTypeMapping, char)) {
        type = cellTypeMapping[char as keyof typeof cellTypeMapping];
      } else {
        type = CellType.PLAYER;
      }
      assert(type)

      mapGrid.push({
        x,
        y,
        type,
      });
    }
  }

  return mapGrid;
}
