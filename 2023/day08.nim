import std/strutils
import std/tables
import std/sequtils
import std/sugar
import std/math

type 
  MapValue = tuple[left: string, right: string]

proc partOne() =
  let
    file = readFile("2023/data/08/input.txt")
    groups = file.split("\n\n")
    directions = groups[0]
    directionsLen = directions.len()
  
  var mapTable = newTable[string, MapValue]()

  for row in groups[1].split("\n"):
    let 
      split = row.split(" = ")
      key = split[0]
      left = split[1].split(", ")[0].replace("(", "")
      right = split[1].split(", ")[1].replace(")", "")
    
    mapTable[key] = (left: left, right: right)

  var 
    location = "AAA"
    steps = 0
    i = 0

  while location != "ZZZ":
    let 
      value = mapTable[location]
      direction = directions[i]
    
    if direction == 'R':
      location = value.right
    elif direction == 'L':
      location = value.left
    i = (i + 1) mod directionsLen
    steps += 1
  
  echo steps

partOne()
echo "=========="

proc partTwo() =
  let
    file = readFile("2023/data/08/input.txt")
    groups = file.split("\n\n")
    directions = groups[0]
    directionsLen = directions.len()
  
  var 
    mapTable = newTable[string, MapValue]()

  for row in groups[1].split("\n"):
    let 
      split = row.split(" = ")
      key = split[0]
      left = split[1].split(", ")[0].replace("(", "")
      right = split[1].split(", ")[1].replace(")", "")
    
    mapTable[key] = (left: left, right: right)
  
  var 
    locations = mapTable.keys.toSeq.filter(key => key.endsWith('A'))
    stepCounts: seq[int] = @[]
    steps = 1

  for location in locations:
    var
      l = location
      steps = 0
      i = 0

    while not l.endsWith('Z'):
      let 
        value = mapTable[l]
        direction = directions[i]
      
      if direction == 'R':
        l = value.right
      elif direction == 'L':
        l = value.left
      i = (i + 1) mod directionsLen
      steps += 1
    
    stepCounts.add(steps)
  
  steps = stepCounts.foldl(lcm(a, b), steps)
  echo steps


  # while not locations.all(l => l.endsWith('Z')):
  #   let direction = directions[i]

  #   locations = locations.map do (l: string) -> string:
  #     let value = mapTable[l]
  #     if direction == 'R':
  #       result = value.right
  #     elif direction == 'L':
  #       result = value.left
    
  #   i = (i + 1) mod directionsLen
  #   steps += 1

partTwo()
