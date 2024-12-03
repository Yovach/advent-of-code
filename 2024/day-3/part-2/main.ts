const fileContent: string = Deno.readTextFileSync("./input.txt").trimEnd();

const handledInstructions: string[] = [];
const regexInstructions = /([a-z]+'?[a-z]+)\((.*?)\)/g;
for (const match of fileContent.matchAll(regexInstructions)) {
  const instruction = match[1].trim();
  if (!handledInstructions.includes(instruction)) {
    handledInstructions.push(instruction);
  }
}

// let total = 0;
// for (const match of fileContent.matchAll(regex)) {
//   const firstNum = parseAsInt(match[1]);
//   const secondNum = parseAsInt(match[2]);
//   total += firstNum * secondNum;

// }
// console.log(total)
