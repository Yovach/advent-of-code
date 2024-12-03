function parseAsInt(val: string) {
  return parseInt(val, 10);
}

const fileContent: string = Deno.readTextFileSync("./input.txt").trimEnd();
const regexInstructions: RegExp = /(do|don't)\(\)|(mul)\((\d+),(\d+)\)/g;

let canExecuteNext = true;
let total = 0;

for (const match of fileContent.matchAll(regexInstructions)) {
  const instructionMethod = match[1] ?? match[2];
  if (instructionMethod === "mul") {
    if (canExecuteNext === false) {
      continue;
    }

    const firstNumber = parseAsInt(match[3]);
    const secondNumber = parseAsInt(match[4]);
    if (!Number.isNaN(firstNumber) && !Number.isNaN(secondNumber)) {
      total += firstNumber * secondNumber;
    }
  } else if (instructionMethod === "do") {
    canExecuteNext = true;
  } else if (instructionMethod === "don't") {
    canExecuteNext = false;
  }
}

console.log(total)
