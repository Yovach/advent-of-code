function parseAsInt(val: string) {
  return parseInt(val, 10);
}

const fileContent: string = Deno.readTextFileSync("./input.txt").trimEnd();

// Aussi appelées "reports"
const lines: string[] = fileContent.split("\n");
let nbValidReports = 0;

for (const line of lines) {
  const levels: number[] = line.split(" ").map(parseAsInt);

  let isIncreasing: boolean | null = null;
  let isValid: boolean = true;
  let previous: number | null = null;

  let nbErrors = 0;

  for (let index = 0; index < levels.length && isValid === true; index++) {
    const value: number = levels[index];

    if (previous === null) {
      previous = value;
      continue;
    }

    const ascending: boolean = (value - previous) > 0;
    if (isIncreasing === null) {
      isIncreasing = ascending;
    }

    if (ascending !== isIncreasing) {
      nbErrors++;
      continue;
    }

    const distance: number = Math.abs(value - previous);
    if (distance < 1 || distance > 3) {
      nbErrors++;
      continue;
    }

    previous = value;
  }

  if (nbErrors <= 1) {
    nbValidReports += 1;
  }
}

console.log(nbValidReports);
