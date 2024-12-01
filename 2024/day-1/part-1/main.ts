const fileContent = Deno.readTextFileSync("./input.txt").trimEnd();

const nbElements = fileContent.split("\n").length;

const regex = /(\d+)\s{1,}(\d+)/g;
const matches = fileContent.matchAll(regex);

const firstColumn: number[] = Array.from({ length: nbElements });
const secondColumn: number[] = Array.from({ length: nbElements });

let index = 0;
for (const match of matches) {
  firstColumn[index] = parseInt(match[1], 10);
  secondColumn[index] = parseInt(match[2], 10);

  index++;
}

const sortedFirstColumn = firstColumn.toSorted((a: number, b: number) => a - b);
const sortedSecondColumn = secondColumn.toSorted((a: number, b: number) => a - b);

let total = 0;
for (index = 0; index < nbElements; index++) {
  total += Math.abs(sortedFirstColumn[index] - sortedSecondColumn[index]);
}

console.log(total);
