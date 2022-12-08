type File = {
  name: string
  size: number
}

class Directory {
  readonly name: string
  parent?: Directory
  directories: Directory[]
  files: File[]
  totalSize: number

  constructor(name: string, parent?: Directory) {
    this.name = name
    this.directories = []
    this.files = []
    this.totalSize = 0

    if (parent !== undefined) {
      this.parent = parent
    }
  }

  add(info: string) {
    const [kind, name] = info.split(" ")
    if (kind === "dir" && !this.directories.find((dir) => dir.name === name)) {
      this.directories.push(new Directory(name, this))
    } else if (!this.files.find((file) => file.name === name)) {
      this.files.push({ name: name, size: Number(kind) })
    }
  }

  size(): number {
    if (this.totalSize !== 0) return this.totalSize

    const fileSum = this.files.reduce((sum, file) => sum + file.size, 0)
    const directorySum = this.directories.reduce(
      (sum, directory) => sum + directory.size(),
      0
    )

    this.totalSize = fileSum + directorySum
    return this.totalSize
  }
}

const partOne = (data: string) => {
  const root = new Directory("root")
  traverse(root, data)

  const sum = sumEligible(root)
  console.log(sum)
}

const partTwo = (data: string) => {
  const root = new Directory("root")
  const eligible: Directory[] = []

  traverse(root, data)
  const freeSpace = 70000000 - root.size()

  addEligible(eligible, root, 30000000 - freeSpace)
  eligible.sort((a, b) => {
    const aSize = a.size()
    const bSize = b.size()
    if (aSize < bSize) {
      return -1
    } else if (aSize > bSize) {
      return 1
    } else {
      return 0
    }
  })

  console.log(eligible[0].size())
}

function traverse(root: Directory, data: string): void {
  const lines = data.split("\n")

  let current = root
  let index = 0

  while (index < lines.length) {
    const [isCommand, command, value] = lines[index].split(" ")

    if (isCommand !== "$") {
      console.log("Your parsing is miserable")
      break
    }

    if (command === "cd" && value === "/") {
      current = root
      index++
    } else if (command === "cd" && value === "..") {
      if (current.parent === undefined) {
        console.log("thing are broken")
        break
      }

      current = current.parent
      index++
    } else if (command === "cd") {
      const child = current.directories.find((dir) => dir.name === value)
      if (child !== undefined) {
        current = child
        index++
      } else {
        console.log("Your parsing is miserable TAKE THREE!")
        break
      }
    } else if (command === "ls") {
      index++
      while (index < lines.length && lines[index][0] !== "$") {
        current.add(lines[index])
        index++
      }
    } else {
      console.log("Your parsing is miserable TAKE TWO")
      break
    }
  }
}

function sumEligible(directory: Directory): number {
  let sum = 0
  const directorySize = directory.size()

  if (directorySize < 100000) {
    sum += directorySize
  }

  sum += directory.directories.reduce(
    (total, current) => total + sumEligible(current),
    0
  )

  return sum
}

function addEligible(
  eligible: Directory[],
  directory: Directory,
  spaceNeeded: number
): void {
  const size = directory.size()
  if (size >= spaceNeeded) {
    eligible.push(directory)
  }

  for (const child of directory.directories) {
    addEligible(eligible, child, spaceNeeded)
  }
}

export { partOne, partTwo }
