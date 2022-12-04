const partOne = (data: string) => {
  const rangePairs = data.split("\n")
  let score = 0

  for (const pair of rangePairs) {
    if (pair === "") continue

    const [firstRange, secondRange] = pair.split(",")
    const firstPoint = rangeToPoint(firstRange)
    const secondPoint = rangeToPoint(secondRange)

    if (
      contains(firstPoint, secondPoint) ||
      contains(secondPoint, firstPoint)
    ) {
      score++
    }
  }

  console.log(score)
}

const partTwo = (data: string) => {
  const rangePairs = data.split("\n")
  let score = 0

  for (const pair of rangePairs) {
    if (pair === "") continue

    const [firstRange, secondRange] = pair.split(",")
    const firstPoint = rangeToPoint(firstRange)
    const secondPoint = rangeToPoint(secondRange)

    if (overlaps(firstPoint, secondPoint)) {
      score++
    }
  }

  console.log(score)
}

function rangeToPoint(rangeString: string): number[] {
  return rangeString.split("-").map((i) => Number(i))
}

function contains(pointA: number[], pointB: number[]): boolean {
  return (
    pointB[0] >= pointA[0] &&
    pointB[0] <= pointA[1] &&
    pointB[1] >= pointA[0] &&
    pointB[1] <= pointA[1]
  )
}

function overlaps(pointA: number[], pointB: number[]): boolean {
  return (
    (pointB[0] <= pointA[0] && pointB[1] >= pointA[0]) ||
    (pointB[0] >= pointA[0] && pointB[0] <= pointA[1])
  )
}

export { partOne, partTwo }
