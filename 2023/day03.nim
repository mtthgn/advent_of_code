import std/strutils
import std/sequtils
import sugar

proc checkLines(numberWithIndex: tuple[number: string, index: int], lines: seq[string], lineIndex: int): int =
  if numberWithIndex.index == -1:
    return 0

  var 
    line = lines[lineIndex]
    indicesToCheck: seq[tuple[lineIndex: int, charIndex:int]] = @[]
    charIndicesToCheck: seq[int] = @[]
    startIndex = numberWithIndex.index
    endIndex = numberWithIndex.index + numberWithIndex.number.len() - 1

  if startIndex > 0:
    charIndicesToCheck.add(startIndex - 1)
    # check the left side
    indicesToCheck.add((lineIndex: lineIndex, charIndex: startIndex - 1))

  for index in startIndex..endIndex:
    charIndicesToCheck.add(index)

  if endIndex < line.len() - 1:
    charIndicesToCheck.add(endIndex + 1)
    # check the right side
    indicesToCheck.add((lineIndex: lineIndex, charIndex: endIndex + 1))

  # Check the indices on the line above
  if lineIndex > 0:
    for index in charIndicesToCheck:
      indicesToCheck.add((lineIndex: lineIndex - 1, charIndex: index))
  
  # Check the indices on the line below
  if lineIndex < lines.len() - 1:
    for index in charIndicesToCheck:
      indicesToCheck.add((lineIndex: lineIndex + 1, charIndex: index))
  
  # Add the number to the total if it isn't surrounded by periods
  # Note: I may have to check if digits are touching as well and exclude those?
  if indicesToCheck.all((i) => lines[i.lineIndex][i.charIndex] == '.') == false:
    return parseInt(numberWithIndex.number)
  else:
    return 0

proc partOne() =
  var finalNumber = 0
  let file = readFile("2023/data/03/input.txt")
  let lines = file.split("\n")

  for lineIndex, line in lines:
    var numberWithIndex: tuple[number: string, index: int]
    numberWithIndex.number = ""
    numberWithIndex.index = -1
    for charIndex, character in line:
      try:
        if parseInt($character) >= 0:
          numberWithIndex.number.add(character)

          if numberWithIndex.index == -1:
            numberWithIndex.index = charIndex  
      except ValueError:
        if numberWithIndex.index != -1:
          # Do the thing
          finalNumber += checkLines(numberWithIndex, lines, lineIndex)

          # Clear out the tuple for subsequent numbers on the line  
          numberWithIndex.number = ""
          numberWithIndex.index = -1
        else:
          continue
    # You've exited the loop, check to see if you ended with a number
    finalNumber += checkLines(numberWithIndex, lines, lineIndex)
  echo finalNumber

type 
  PartNumber = object
    numString: string
    numInt, startIndex, endIndex: int

# Adjacent is meant for adjacency checks on the lines above
# and below the part
proc adjacent(part: PartNumber, index: int): bool =
  index >= part.startIndex - 1 and index <= part.endIndex + 1

# Confusing terminolgy because of nextTo is also "adjacent" technically
# but nextTo is meant for checks on the same line.
proc nextTo(part: PartNumber, index: int): bool =
  index == part.startIndex - 1 or index == part.endIndex + 1

proc partTwo() =
  var 
    finalNumber = 0
    linePartNumbers: seq[seq[PartNumber]] = @[]
    lineGears: seq[seq[int]] = @[]

  let 
    file = readFile("2023/data/03/input.txt")
    lines = file.split("\n")
    
  for lineIndex, line in lines:
    var 
      partNumbers: seq[PartNumber] = @[]
      gears: seq[int] = @[]
      numberWithIndex: tuple[number: string, index: int]

    numberWithIndex.number = ""
    numberWithIndex.index = -1
    for charIndex, character in line:
      try:
        if parseInt($character) >= 0:
          numberWithIndex.number.add(character)

          if numberWithIndex.index == -1:
            numberWithIndex.index = charIndex  
      except ValueError:
        if numberWithIndex.index != -1:
          partNumbers.add(PartNumber(
            numString: numberWithIndex.number,
            numInt: parseInt(numberWithIndex.number),
            startIndex: numberWithIndex.index,
            endIndex: numberWithIndex.index + numberWithIndex.number.len() - 1
          ))

          # Clear out the tuple for subsequent numbers on the line  
          numberWithIndex.number = ""
          numberWithIndex.index = -1
        if character == '*':
          gears.add(charIndex)

    # You've exited the loop, check to see if you ended with a number
    if numberWithIndex.index != -1:
      partNumbers.add(PartNumber(
        numString: numberWithIndex.number,
        numInt: parseInt(numberWithIndex.number),
        startIndex: numberWithIndex.index,
        endIndex: numberWithIndex.index + numberWithIndex.number.len() - 1
      ))
    
    linePartNumbers.add(partNumbers)
    lineGears.add(gears)

  for lineIndex, gears in lineGears:
    for gearIndex in gears:
      var adjacentParts: seq[PartNumber] = @[]
      if lineIndex > 0:
        for part in linePartNumbers[lineIndex - 1]:
          if part.adjacent(gearIndex):
            adjacentParts.add(part)
      
      for part in linePartNumbers[lineIndex]:
        if part.nextTo(gearIndex):
          adjacentParts.add(part)
      
      if lineIndex < lines.len() - 1:
        for part in linePartNumbers[lineIndex + 1]:
          if part.adjacent(gearIndex):
            adjacentParts.add(part)
      
      if adjacentParts.len() == 2:
        finalNumber += (adjacentParts[0].numInt * adjacentParts[1].numInt)
  echo finalNumber

partTwo()