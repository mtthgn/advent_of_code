const partOne = (data: string) => {
  const buffer: string[] = []

  let i = 0
  while (i < data.length) {
    const character = data[i]

    if (buffer.length < 4) {
      buffer.push(character)
    } else if (buffer.length === 4) {
      const bufferAsSet = new Set(buffer)
      if (bufferAsSet.size === buffer.length) {
        break
      } else {
        buffer.shift()
        buffer.push(character)
      }
    } else {
      console.log("I messed up, buffer length should never be more than 4")
      break
    }

    i++
  }

  console.log(`index: ${i}`)
}

const partTwo = (data: string) => {
  const buffer: string[] = []

  let i = 0
  while (i < data.length) {
    const character = data[i]

    if (buffer.length < 14) {
      buffer.push(character)
    } else if (buffer.length === 14) {
      const bufferAsSet = new Set(buffer)
      if (bufferAsSet.size === buffer.length) {
        break
      } else {
        buffer.shift()
        buffer.push(character)
      }
    } else {
      console.log("I messed up, buffer length should never be more than 4")
      break
    }

    i++
  }

  console.log(`index: ${i}`)
}

export { partOne, partTwo }
