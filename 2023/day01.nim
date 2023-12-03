import std/strutils

proc add_number_char(str: ref string, number_char: char) =
  if len(str[]) == 0 or len(str[]) == 1:
    add(str[], number_char)
  else:
    str[1] = number_char

proc add_number_string(str: ref string, line: string, index: int) =
  if line.find("one", index) == index:
    add_number_char(str, '1')
  elif line.find("two", index) == index:
    add_number_char(str, '2')
  elif line.find("three", index) == index:
    add_number_char(str, '3')
  elif line.find("four", index) == index:
    add_number_char(str, '4')
  elif line.find("five", index) == index:
    add_number_char(str, '5')
  elif line.find("six", index) == index:
    add_number_char(str, '6')
  elif line.find("seven", index) == index:
    add_number_char(str, '7')
  elif line.find("eight", index) == index:
    add_number_char(str, '8')
  elif line.find("nine", index) == index:
    add_number_char(str, '9')
  elif line.find("zero", index) == index:
    add_number_char(str, '0')

proc partOne() =
  var finalNumber = 0
  let f = open("2023/data/01/input.txt")
  defer: f.close()

  while true:
    try:
      let line = f.readLine()    
      var number: ref string
      new(number)
      number[] = ""

      for character in line:
        try:
          if parseInt($character) >= 0 :
            add_number_char(number, character)
        except ValueError:
          continue
      
      if len(number[]) == 1:
        add(number[], number[0])
      
      finalNumber += parseInt(number[])
    except EOFError:
      break
  echo finalNumber

proc partTwo() =
  var finalNumber = 0
  let f = open("2023/data/01/input.txt")
  defer: f.close()
  
  while true:
    try:
      let line = f.readLine()    
      var number: ref string
      new(number)
      number[] = ""

      for index, character in line:
        try:
          if parseInt($character) >= 0 :
            add_number_char(number, character)
        except ValueError:
          add_number_string(number, line, index)
          continue

      if len(number[]) == 1:
        add(number[], number[0])
      
      finalNumber += parseInt(number[])
    except EOFError:
      break

  echo finalNumber

partTwo()