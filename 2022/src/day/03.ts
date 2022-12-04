const partOne = (data: string) => {
  const rucksacks = data.split("\n")
  let score = 0

  for (let rucksack of rucksacks) {
    const midpoint = rucksack.length / 2
    const first = rucksack.substring(0, midpoint)
    const firstSet = new Set([...first])
    const second = rucksack.substring(midpoint)
    const matchSet = new Set()

    for (const item of second) {
      if (firstSet.has(item) && !matchSet.has(item)) {
        matchSet.add(item)
        const priority = priorityCode(item)
        score += priority
      }
    }
  }

  console.log(score)
}

const partTwo = (data: string) => {
  const rucksacks = data.split("\n")
  let group: string[] = []
  let score = 0

  for (let rucksack of rucksacks) {
    group.push(rucksack)

    if (group.length === 3) {
      const overlap = group
        .map((sack) => new Set(sack))
        .reduce((intersection, current) =>
          setIntersection(intersection, current)
        )
      score += priorityCode([...overlap][0])
      group = []
    }
  }

  console.log(score)
}

function setIntersection<T>(setA: Set<T>, setB: Set<T>): Set<T> {
  const intersection: Set<T> = new Set()

  for (const item of setB) {
    if (setA.has(item)) {
      intersection.add(item)
    }
  }

  return intersection
}

const priorityCode = (character: string) => {
  const charCode = character.charCodeAt(0)
  let priority = 0

  if (charCode > 96) {
    priority = charCode - 96
  } else {
    priority = charCode - 38
  }

  return priority
}

export { partOne, partTwo }
