# Make a sequence of the matches on the right side, then the number of points is 2^(matches.len() - 1)
import std/strutils
import std/sets
import std/math
import std/sugar

proc partOne() =
  var finalNumber = 0
  let f = open("2023/data/04/input.txt")
  defer: f.close()

  while true:
    try:
      let 
        line = f.readLine()
        card = line.split(" | ")
        winningNumbers = toHashSet(card[0].split(": ")[1].splitWhitespace())
        numbers = toHashSet(card[1].splitWhitespace())
        matches = winningNumbers * numbers # * for sets is intersection

      if matches.len() > 0:
        finalNumber += 2 ^ (matches.len() - 1)

    except EOFError:
      break
  echo finalNumber

proc partTwo() =
  var finalNumber = 0
  let 
    file = readFile("2023/data/04/input.txt")
    lines = file.split("\n")
  var copies: seq[int] = newSeq[int](lines.len())
  
  for i in 0..(lines.len() - 1):
    copies[i] = 1

  for line in lines:
    let
      card = line.split(" | ")
      cardId = parseInt(card[0].split(": ")[0].splitWhitespace()[1])
      winningNumbers = toHashSet(card[0].split(": ")[1].splitWhitespace())
      numbers = toHashSet(card[1].splitWhitespace())
      matches = winningNumbers * numbers # * for sets is intersection
      multiplier = copies[(cardId - 1)]

    if matches.len() > 0:
      for i in 0..(matches.len() - 1):
        copies[cardId + i] += multiplier
  
  for count in copies:
    finalNumber += count

  echo finalNumber

partTwo()