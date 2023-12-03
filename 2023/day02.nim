import std/strutils

proc partOne() =
  var finalNumber = 0
  let f = open("2023/data/02/input.txt")
  defer: f.close()
  while true:
    try:
      let
        line = f.readLine() 
        splitLine = line.split(": ")
        game = splitLine[0]
        gameId = parseInt(game.split(" ")[1])
        rounds = splitLine[1].split("; ")
      
      var 
        blue, green, red = 0
      
      for round in rounds:
        for colorCount in round.split(", "):
          let
            split = colorCount.split(" ")
            count = parseInt(split[0])
            color = split[1]
          
          if color == "blue" and count > blue:
            blue = count
          elif color == "green" and count > green:
            green = count
          elif color == "red" and count > red:
            red = count
      
      if blue <= 14 and green <= 13 and red <= 12:
        finalNumber += gameId

    except EOFError:
      break
  echo finalNumber

proc partTwo() =
  var finalNumber = 0
  let f = open("2023/data/02/input.txt")
  defer: f.close()
  while true:
    try:
      let
        line = f.readLine() 
        splitLine = line.split(": ")
        game = splitLine[0]
        gameId = parseInt(game.split(" ")[1])
        rounds = splitLine[1].split("; ")
      
      var 
        blue, green, red, power = 0
      
      for round in rounds:
        for colorCount in round.split(", "):
          let
            split = colorCount.split(" ")
            count = parseInt(split[0])
            color = split[1]
          
          if color == "blue" and count > blue:
            blue = count
          elif color == "green" and count > green:
            green = count
          elif color == "red" and count > red:
            red = count

      power = blue * green * red
      finalNumber += power

    except EOFError:
      break
  echo finalNumber

partTwo()
