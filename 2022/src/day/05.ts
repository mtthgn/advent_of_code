const partOne = (data: string) => {
  const [stackString, instructionString] = data.split("\n\n")
  const stacks = constructStacks(stackString.split("\n"))
  const instructions = instructionString.split("\n")
  const regex = /move (?<move>\d+) from (?<from>\d+) to (?<to>\d+)/
  let answer = ""

  for (let instruction of instructions) {
    if (instruction === "") continue

    const match = instruction.match(regex)
    if (match === null) continue

    const move = Number(match[1])
    const fromStack = Number(match[2]) - 1
    const toStack = Number(match[3]) - 1

    for (let i = 0; i < move; i++) {
      const crate = stacks[fromStack].pop()!
      stacks[toStack].push(crate)
    }
  }

  for (const stack of stacks) {
    answer += stack.pop()
  }

  console.log(`Answer: ${answer}`)
}

const partTwo = (data: string) => {
  const [stackString, instructionString] = data.split("\n\n")
  const stacks = constructStacks(stackString.split("\n"))
  const instructions = instructionString.split("\n")
  const regex = /move (?<move>\d+) from (?<from>\d+) to (?<to>\d+)/
  let answer = ""

  for (let instruction of instructions) {
    if (instruction === "") continue

    const match = instruction.match(regex)
    if (match === null) continue

    const move = Number(match[1])
    const fromStack = Number(match[2]) - 1
    const toStack = Number(match[3]) - 1

    const crates = stacks[fromStack].splice(-move)
    stacks[toStack].push(...crates)
  }

  for (const stack of stacks) {
    answer += stack.pop()
  }

  console.log(`Answer: ${answer}`)
}

function constructStacks(data: string[]): string[][] {
  const stacks: string[][] = []
  const stackNumbers = data.pop()!.trim()
  const stackCount = Number(stackNumbers[stackNumbers.length - 1])

  for (let i = 0; i < stackCount; i++) {
    stacks.push([])
  }

  for (let i = data.length - 1; i >= 0; i--) {
    const line = data[i]
    let iteration = 0

    for (let c = 1; c < line.length; c += 4) {
      const character = line[c]
      if (character !== undefined && character !== " ") {
        const stack = stacks[iteration]
        stack.push(character)
      }
      iteration++
    }
  }

  return stacks
}

export { partOne, partTwo }
