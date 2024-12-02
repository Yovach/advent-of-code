function parseAsInt(val: string) {
  return parseInt(val, 10);
}

function isValid(report: number[]) {
  let isIncreasing: boolean | null = null;
  let previous: number | null = null;

  let nbErrors = 0;
  for (let index = 0; index < report.length; index++) {
    const value = report[index];
    if (previous === null) {
      previous = value;
      continue;
    }

    const ascending = (value - previous) > 0;
    if (isIncreasing === null) {
      isIncreasing = ascending;
    }

    if (ascending !== isIncreasing) {
      nbErrors++;
      continue;
    }


    isIncreasing = ascending;

    const distance = Math.abs(value - previous);
    if (distance < 1 || distance > 3) {
      nbErrors++;
      continue;
    }

    previous = value;
  }

  return nbErrors <= 1;
}

const fileContent: string = Deno.readTextFileSync("./input.txt").trimEnd();

// Aussi appelées "reports"
const lines: string[] = fileContent.split("\n");
const validReports: string[] = [];

for (let reportsIndex = 0; reportsIndex < lines.length; reportsIndex++) {
  const line = lines[reportsIndex];

  const levels: number[] = line.split(" ").map(parseAsInt);

  if (isValid(levels)) {
    validReports.push(line);
  }
}

console.log(validReports.length);
