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

let total = 0;
for (const value of firstColumn) {
  total += secondColumn.filter((val) => val === value).length * value;
}

console.log(total)
