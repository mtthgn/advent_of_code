# NOTE: I changed the format so to "time distance" for 
#       each race living on the same line. I'm not a monster
#       I swear
import std/sequtils
import std/strutils
import std/sugar
import std/math

type
  Race = tuple[time: int, distance: int]

# IDEA
# x = milliseconds held down,  velocity of the boat in mm/s
# distance = x * (time - x)
# distance = time * x - x^2
# x^2 + distance = time * x
# x^2 - time*x + distance = 0 
# AKA QUADRATIC FORMULA
proc runRace(fileName: string) =
  let 
    file = readFile(fileName)
    lines = file.split("\n")
    races: seq[Race] = lines.map(line => (time: parseInt(line.split(" ")[0]), distance: parseInt(line.split(" ")[1])))

  var winningCounts: seq[int] = @[]

  for race in races:
    var min = ((float(race.time) - sqrt(float(((race.time * -1) ^ 2) - (4 * race.distance))))/ 2)
    var max = ((float(race.time) + sqrt(float(((race.time * -1) ^ 2) - (4 * race.distance))))/ 2)

    if min.formatBiggestFloat(ffDecimal, 4).split(".")[1].all(c => c == '0'):
      min += 1
    
    if max.formatBiggestFloat(ffDecimal, 4).split(".")[1].all(c => c == '0'):
      max -= 1

    min = ceil(min)
    max = floor(max)

    winningCounts.add(int(max) + 1 - int(min))
    # echo "MIN: ", int(min)
    # echo "MAX: ", int(max)
    # echo "Winning moves:", winning

  echo winningCounts.foldl(a * b, 1)

proc partOne() =
  runRace("2023/data/06/input.txt")

proc partTwo() =
  runRace("2023/data/06/input_part_two.txt")

partOne()
partTwo()