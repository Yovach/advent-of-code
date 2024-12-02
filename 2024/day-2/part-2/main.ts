const fileContent = Deno.readTextFileSync("./input.txt").trimEnd();

// Aussi appelées "reports"
const lines: string[] = fileContent.split("\n");
let nbValidReports = 0;

for (const line of lines) {
  const levels = line.split(" ");

  let isIncreasing: boolean | null = null;
  let isValid: boolean = true;
  let previous: number | null = null;

  for (let index = 0; index < levels.length && isValid === true; index++) {
    const value = levels[index];
    const valueAsInt = parseInt(value, 10);

    if (previous === null) {
      previous = valueAsInt;
      continue;
    }

    const ascending = (valueAsInt - previous) > 0;
    if (isIncreasing === null) {
      isIncreasing = ascending;
    }

    if (ascending !== isIncreasing) {
      isValid = false;
      continue;
    }

    const distance = Math.abs(valueAsInt - previous);
    if (distance < 1 || distance > 3) {
      isValid = false;
      continue;
    }

    previous = valueAsInt;
  }

  if (isValid) {
    nbValidReports += 1;
  }
}

console.log(nbValidReports);
