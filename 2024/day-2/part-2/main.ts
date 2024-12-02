function parseAsInt(val: string) {
  return parseInt(val, 10);
}

function getErroredIndexes(report: number[], ignoredIndex: number | null = null): number[] {
  const erroredIndexes: number[] = [];

  let isIncreasing: boolean | null = null;
  let previous: number | null = null;

  for (let index = report.length - 1; index >= 0; index--) {
    // On ne compte pas l'erreur de l'index ignoré
    if (ignoredIndex !== null && ignoredIndex === index) {
      continue;
    }

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
      erroredIndexes.push(index);
      continue;
    }

    previous = value;
  }

  return erroredIndexes;
}

const fileContent: string = Deno.readTextFileSync("./input.txt").trimEnd();

// Aussi appelées "reports"
const lines: string[] = fileContent.split("\n");
const validReports: string[] = [];

// On parcourt les reports
for (let reportsIndex = 0; reportsIndex < lines.length; reportsIndex++) {
  const line = lines[reportsIndex];

  // On récupère les éléments d'un rapport
  const levels: number[] = line.split(" ").map(parseAsInt);

  // Récupérer les indexes en erreur
  const erroredIndexes = getErroredIndexes(levels);
  if (erroredIndexes.length === 0) {
    validReports.push(line);
    continue;
  }


  let foundGoodReport = false;
  // On parcourt les index erronés
  for (let index = 0; index < erroredIndexes.length && foundGoodReport === false; index++) {
    const nbRemainingErrors = getErroredIndexes(levels, erroredIndexes[index]);
    if (nbRemainingErrors.length === 0) {
      validReports.push(line);
      foundGoodReport = true;
    }
  }
}

console.log(validReports.length);
