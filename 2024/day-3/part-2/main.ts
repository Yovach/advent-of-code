function parseAsInt(val: string) {
  return parseInt(val, 10);
}

type Instruction = {
  method: string;
  arguments: string;
}

const fileContent: string = Deno.readTextFileSync("./input.txt").trimEnd();

const regexInstructions = /([a-z]+'?[a-z]+)\((.*?)\)/g;

let canExecuteNext = true;

let total = 0;

for (const match of fileContent.matchAll(regexInstructions)) {
  const instructionMethod = match[1];
  console.log(instructionMethod)

  if (instructionMethod === "mul") {
    if (canExecuteNext === false) {
      continue;
    }

    const instructionArguments = match[2];
    const args = instructionArguments.split(",").map(parseAsInt);
    if (args.length === 2 && args.every((value) => !Number.isNaN(value))) {
      total += args[0] * args[1]
    }
  } else if (instructionMethod === "do") {
    canExecuteNext = true;
  } else if (instructionMethod === "don't") {
    canExecuteNext = false;
  }
}

console.log(total)
