function parseAsInt(val: string) {
  return parseInt(val, 10);
}

function isValidSequence(report: number[]): boolean {
  let isIncreasing: boolean | null = null;
  let previous: number | null = null;

  for (let index = 0; index < report.length; index++) {
    const value: number = report[index];

    // S'il n'y a pas d'élément précédent, on assigne la valeur actuelle
    // et on continue la boucle
    if (previous === null) {
      previous = value;
      continue;
    }

    // est-ce que la suite est montante ou descendante?
    const ascending: boolean = (value - previous) > 0;
    if (isIncreasing === null) {
      isIncreasing = ascending;
    }

    // récupérer la distance absolue de A-B
    const distance: number = Math.abs(value - previous);
    if (ascending !== isIncreasing || distance < 1 || distance > 3) {
      return false;
    }

    previous = value;
  }

  return true;
}

const fileContent: string = Deno.readTextFileSync("./input.txt").trimEnd();

// Aussi appelées "reports"
const lines: string[] = fileContent.split("\n");
let nbValidReports = 0;

// On parcourt les reports
for (const line of lines) {
  // On récupère les éléments d'un rapport
  const levels: number[] = line.split(" ").map(parseAsInt);

  // Récupérer les indexes en erreur
  const isValid = isValidSequence(levels);
  if (isValid === true) {
    nbValidReports++;
    continue;
  }


  let hasGoodReport = false;
  // On parcourt les index erronés
  for (let idx = 0; idx < levels.length && hasGoodReport === false; idx++) {
    if (isValidSequence(levels.toSpliced(idx, 1))) {
      nbValidReports++;
      hasGoodReport = true;
    }
  }
}

console.log(nbValidReports);
