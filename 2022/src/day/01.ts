const partOne = (data: string) => {
  const chunks = data.split("\n\n")

  let mostCalories = 0
  for (let elf of chunks) {
    const totalCalories = elf
      .split("\n")
      .map((calorieString) => Number(calorieString))
      .reduce((sum, current) => sum + current, 0)

    if (totalCalories > mostCalories) {
      mostCalories = totalCalories
    }
  }
  console.log(mostCalories)
}

const partTwo = (data: string) => {
  const chunks = data.split("\n\n")
  let topThree: number[] = []

  for (let elf of chunks) {
    const calorieSum = elf
      .split("\n")
      .map((calorieString) => Number(calorieString))
      .reduce((sum, current) => sum + current, 0)

    if (topThree.length < 3) {
      topThree.push(calorieSum)
    } else if (calorieSum > topThree[0] && topThree.length == 3) {
      topThree.shift()
      topThree.push(calorieSum)
      topThree = topThree.sort((a: number, b: number) => a - b)
    }
  }

  console.log(topThree.reduce((sum, current) => sum + current, 0))
}

export { partOne, partTwo }
