import { promises as fs } from "fs"
import path from "path"
import { fileURLToPath } from "url"

const advent_day = process.argv[2]
const filename = process.argv[3] || "default"
const part = process.argv[4] || "part_one"
const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

import days, { Day } from "./day/index.js"

async function run(day: Day) {
  if (day === undefined) {
    console.log(`Unrecognized day: ${advent_day}`)
    return
  }
  const hrstart = process.hrtime()
  const filepath = `${__dirname}/../data/${advent_day}/${filename}.txt`
  const data = await fs.readFile(filepath)

  switch (part) {
    case "part_one":
      day.partOne(data.toString())
      break
    case "part_two":
      day.partTwo(data.toString())
      break
  }
  const hrend = process.hrtime(hrstart)

  console.info("Execution time: %ds %dms", hrend[0], hrend[1] / 1000000)
}

await run(days[advent_day])
