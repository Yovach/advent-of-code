import { assert } from "@std/assert";

export enum CellType {
  EMPTY = "EMPTY",
  OBSTACLE = "OBSTACLE",
  PLAYER = "PLAYER",
}

enum Direction {
  NORTH = "NORTH",
  EAST = "EAST",
  SOUTH = "SOUTH",
  WEST = "WEST",
}

export type GridCell = {
  x: number;
  y: number;
  type: CellType;
  isInitialStart: boolean;
};

const CELL_TYPE_MAPPING = Object.freeze({
  ".": CellType.EMPTY,
  "#": CellType.OBSTACLE,
});

const PLAYER_DIRECTION = Object.freeze({
  "^": Direction.NORTH,
  ">": Direction.EAST,
  "v": Direction.SOUTH,
  "<": Direction.WEST,
});

export type Player = {
  x: number;
  y: number;
  direction: keyof typeof PLAYER_DIRECTION;
};

function getNextPosition(player: Player): [x: number, y: number] {
  if (player.direction === "^") {
    return [player.x, player.y - 1];
  } else if (player.direction === ">") {
    return [player.x + 1, player.y];
  } else if (player.direction === "v") {
    return [player.x, player.y + 1];
  } else if (player.direction === "<") {
    return [player.x - 1, player.y];
  }

  throw new Error("Invalid player direction");
}

function canMoveForward(mapGrid: GridCell[], player: Player): boolean {
  const [x, y] = getNextPosition(player);
  const nextGrid = mapGrid.find((grid) => grid.x === x && grid.y === y);
  if (!nextGrid) {
    return false;
  }
  return nextGrid.type === CellType.EMPTY;
}

export function moveForward(mapGrid: GridCell[], player: Player) {
  console.log(player);
  let canMove;
  do {
    canMove = canMoveForward(mapGrid, player);

    if (!canMove) {
      player.direction = rotate(player);
    }

    const [nextX, nextY] = getNextPosition(player);
    player.x = nextX;
    player.y = nextY;
    console.log(player);
  } while (canMove === true);

  return true;
}

function rotate(player: Player): Player["direction"] {
  let direction: Player["direction"];

  switch (player.direction) {
    case "^":
      direction = ">";
      break;
    case ">":
      direction = "v";
      break;
    case "v":
      direction = "<";
      break;
    case "<":
      direction = "^";
      break;
  }

  return direction;
}

export function getGridFromFile(
  fileContent: string,
): { grid: GridCell[]; player: Player | null } {
  const mapGrid: GridCell[] = [];
  const lines = fileContent.split("\n");
  let player: Player | null = null;
  for (let y = 0; y < lines.length; y++) {
    const line = lines.at(y);
    assert(line);

    for (let x = 0; x < line.length; x++) {
      const char = line.at(x);
      assert(char);

      let isInitialStart = false;
      let type: CellType;
      if (Object.hasOwn(CELL_TYPE_MAPPING, char)) {
        type = CELL_TYPE_MAPPING[char as keyof typeof CELL_TYPE_MAPPING];
      } else {
        assert(Object.keys(PLAYER_DIRECTION).includes(char));

        type = CellType.EMPTY;
        isInitialStart = true;

        player = {
          x,
          y,
          direction: char as Player["direction"],
        };
      }
      assert(type);

      mapGrid.push({
        x,
        y,
        type,
        isInitialStart,
      });
    }
  }

  return { grid: mapGrid, player };
}

export function getInitialPlayerPosition(mapGrid: GridCell[]) {
  return mapGrid.find((val) => val.isInitialStart === true);
}
