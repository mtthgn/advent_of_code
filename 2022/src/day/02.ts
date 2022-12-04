const partOne = (data: string) => {
  const matches = data.split("\n")
  let score = 0

  for (let match of matches) {
    switch (match) {
      case "A X":
        score += 4
        break
      case "A Y":
        score += 8
        break
      case "A Z":
        score += 3
        break
      case "B X":
        score += 1
        break
      case "B Y":
        score += 5
        break
      case "B Z":
        score += 9
        break
      case "C X":
        score += 7
        break
      case "C Y":
        score += 2
        break
      case "C Z":
        score += 6
        break
    }
  }

  console.log(`Score: ${score}`)
}

const partTwo = (data: string) => {
  const matches = data.split("\n")
  let score = 0

  for (let match of matches) {
    switch (match) {
      case "A X":
        score += 3
        break
      case "A Y":
        score += 4
        break
      case "A Z":
        score += 8
        break
      case "B X":
        score += 1
        break
      case "B Y":
        score += 5
        break
      case "B Z":
        score += 9
        break
      case "C X":
        score += 2
        break
      case "C Y":
        score += 6
        break
      case "C Z":
        score += 7
        break
    }
  }
  console.log(`Score: ${score}`)
}

export { partOne, partTwo }
