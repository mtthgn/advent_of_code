import std/strutils
import std/sequtils
import std/sugar
import std/sets

# Finite Differences
# ------------------
# First Order:  y = mx + b
# Second Order: y = ax^2 + bx + c
# Third Order:  y = ax^3 + bx^2 + cx + d
#
# Not sure how to capture those in code other than diff sequences count == order, 
# but solving for a, b, c, and d seems like more work than just naively going back up
# the list.

proc difference(list: seq[int]): ref seq[int] =
  var 
    differenceList = new seq[int]
    
    i = 0
  
  while i < list.len - 1:
    differenceList[].add(list[i + 1] - list[i])
    i += 1
  
  result = differenceList

proc differences(list: seq[int]): seq[ref seq[int]] =
  var
    diffsList: seq[ref seq[int]] = @[]
    workingList = list.difference()

  diffsList.add(workingList)

  while toHashSet(workingList[]).len != 1:
    workingList = workingList[].difference()
    diffsList.add(workingList)
  
  result = diffsList



proc partOne() =
  let 
    file = readFile("2023/data/09/input.txt")
    lines = file.split("\n").map(l => l.split(" ").map(i => parseInt(i)))

  var  
    diffGroups = lines.map(l => l.differences)
    finalNumber = 0

  for diffGroup in diffGroups:
    var i = diffGroup.len - 1

    while i >= 1:
      var 
        newDiff = diffGroup[i- 1][^1] + diffGroup[i][^1]
        group = diffGroup[i - 1]
        
      group[].add(newDiff)
      i -= 1

  var predictions = diffGroups.map(group => group[0][^1])

  var i = 0
  while i < predictions.len():
    finalNumber += lines[i][^1] + predictions[i]
    i += 1

  echo finalNumber
    
partOne()

echo "======"

proc partTwo() =
  let 
    file = readFile("2023/data/09/input.txt")
    lines = file.split("\n").map(l => l.split(" ").map(i => parseInt(i)))

  var  
    diffGroups = lines.map(l => l.differences)
    finalNumber = 0

  for diffGroup in diffGroups:
    var i = diffGroup.len - 1

    while i >= 1:
      var 
        newDiff = diffGroup[i - 1][0] - diffGroup[i][0]
        group = diffGroup[i - 1]
        
      group[].insert(newDiff, 0)
      i -= 1

  var predictions = diffGroups.map(group => group[0][0])
  
  var i = 0
  while i < predictions.len():
    finalNumber += lines[i][0] - predictions[i]
    i += 1

  echo finalNumber

partTwo()