function parseAsInt(val: string) {
  return parseInt(val, 10);
}

const fileContent: string = Deno.readTextFileSync("./input.txt").trimEnd();

const regex = /mul\((\d+),(\d+)\)/gm;

let total = 0;
for (const match of fileContent.matchAll(regex)) {
  const firstNum = parseAsInt(match[1]);
  const secondNum = parseAsInt(match[2]);
  total += firstNum * secondNum;

}
console.log(total)
