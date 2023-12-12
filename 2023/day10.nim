import std/strutils
import std/sequtils
import std/sugar

type
  Point = tuple[row: int, column: int]
  Direction = enum
    Above, Below, Left, Right
  Pipe = enum
    Horizontal = '-'
    Ground = '.'
    SouthToWest = '7'
    SouthToEast = 'F'
    NorthToWest = 'J'
    NorthToEast = 'L'
    Start = 'S'
    Vertical = '|'

# proc connects(a: Pipe, b: Pipe, direction: Direction):

proc pipeAt(sequence: seq[string], point: Point): Pipe =
  if point.row < 0 or point.row >= sequence.len:
    result = Ground
  elif point.column < 0 or point.column >= sequence[0].len:
    result = Ground
  else:
    result = Pipe(sequence[point.row][point.column])
  
proc connectsAbove(a: Pipe, b: Pipe): bool =
  case a:
    of Horizontal, SouthToWest, SouthToEast, Ground:
      result = false
    of Vertical, NorthToEast, NorthToWest, Start:
      result = @[SouthToWest, SouthToEast, Vertical, Start].any(pipe => pipe == b)

proc connectsBelow(a: Pipe, b: Pipe): bool =
  case a:
    of Horizontal, NorthToEast, NorthToWest, Ground:
      result = false
    of Vertical, SouthToEast, SouthToWest, Start:
      result = @[NorthToEast, NorthToWest, Vertical, Start].any(pipe => pipe == b)

proc connectsLeft(a: Pipe, b: Pipe): bool =
  case a:
    of Vertical, NorthToEast, SouthToEast, Ground:
      result = false
    of Horizontal, NorthToWest, SouthToWest, Start:
      result = @[NorthToEast, SouthToEast, Horizontal, Start].any(pipe => pipe == b)

proc connectsRight(a: Pipe, b: Pipe): bool =
  case a:
    of Vertical, NorthToWest, SouthToWest, Ground:
      result = false
    of Horizontal, NorthToEast, SouthToEast, Start:
      result = @[NorthToWest, SouthToWest, Horizontal, Start].any(pipe => pipe == b)

proc partOne() =
  let
    file = readFile("2023/data/10/input.txt")
    lines = file.split("\n")
    sLocation = file.replace("\n", "").find("S")
    columnLength = lines[0].len
    start: Point = (row: sLocation div columnLength, column: sLocation mod columnLength) 

  var
    location: Point
    previous: Point
    steps = 1
  
  # Check above
  if @[Pipe.Vertical, Pipe.SouthToEast, Pipe.SouthToWest].any(pipe => pipe == Pipe(lines[start.row - 1][start.column])):
    location = (row: start.row - 1, column: start.column)
    previous = start
  # Check below
  elif @[Pipe.Vertical, Pipe.NorthToEast, Pipe.NorthToWest].any(pipe => pipe == Pipe(lines[start.row + 1][start.column])):
    location = (row: start.row + 1, column: start.column)
    previous = start
  # Check left
  elif @[Pipe.Horizontal, Pipe.NorthToEast, Pipe.SouthToEast].any(pipe => pipe == Pipe(lines[start.row][start.column - 1])):
    location = (row: start.row, column: start.column - 1)
    previous = start
  # Check right
  elif @[Pipe.Horizontal, Pipe.NorthToWest, Pipe.SouthToWest].any(pipe => pipe == Pipe(lines[start.row][start.column + 1])):
    location = (row: start.row, column: start.column + 1)
    previous = start

  echo "Start: ", previous
  echo "Current: ", location

  while location != start:
    let 
      pipe = lines.pipeAt(location)
      above: Point = (row: location.row - 1, column: location.column)
      below: Point = (row: location.row + 1, column: location.column)
      left: Point = (row: location.row, column: location.column - 1)
      right: Point = (row: location.row, column: location.column + 1)

    if above != previous and pipe.connectsAbove(lines.pipeAt(above)):
      previous = location
      location = above
      steps += 1
    elif below != previous and pipe.connectsBelow(lines.pipeAt(below)):
      previous = location
      location = below
      steps += 1
    elif left != previous and pipe.connectsLeft(lines.pipeAt(left)):
      previous = location
      location = left
      steps += 1
    elif right != previous and pipe.connectsRight(lines.pipeAt(right)):
      previous = location
      location = right
      steps += 1
    else:
      echo "Something broke"
      quit(QuitFailure)

  echo "total steps: ", steps
  echo "midpoint: ", steps div 2


partOne()

echo "===="

proc partTwo() =
  echo "lol"

partTwo()