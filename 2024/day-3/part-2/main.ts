function parseAsInt(val: string) {
  return parseInt(val, 10);
}

type Instruction = {
  method: string;
  arguments: string;
}

const fileContent: string = Deno.readTextFileSync("./input.txt").trimEnd();

const handledInstructions = ["(do)\\(\\)", "(don't)\\(\\)", "(mul)\\((\\d+),(\\d+)\\)"]

const regexInstructions = new RegExp("(" + handledInstructions.join("|") + ")", "g")
let canExecuteNext = true;

let total = 0;

for (const match of fileContent.matchAll(regexInstructions)) {
  const instructionMethod = match[1];
  if (instructionMethod.startsWith("mul")) {
    if (canExecuteNext === false) {
      continue;
    }

    const firstNumber = parseAsInt(match[5]);
    const secondNumber = parseAsInt(match[6]);
    if (!Number.isNaN(firstNumber) && !Number.isNaN(secondNumber)) {
      total += firstNumber * secondNumber;
    }
  } else if (instructionMethod.startsWith("don't")) {
    canExecuteNext = false;
  } else if (instructionMethod.startsWith("do")) {
    canExecuteNext = true;
  }
}

console.log(total)
