import std/strutils
import std/sequtils
import std/algorithm
import std/sugar

type
  SeedRange = tuple[start: int, length: int]
  TranslationRange = tuple[source: int, destination: int, range: int]

proc generateTranslation(dataString: string): seq[TranslationRange] =
  var translation: seq[TranslationRange] = @[]
  let data = dataString.split(":\n")[1].split("\n")
  for row in data:
      let
        rowData = row.splitWhitespace()
        destination = parseInt(rowData[0])
        source = parseInt(rowData[1])
        range = parseInt(rowData[2])
      translation.add((source: source, destination: destination, range: range))
  translation

proc partOne() =
  var 
    translations: seq[seq[tuple[source: int, destination: int, range: int]]] = @[]
  let 
    file = readFile("2023/data/05/input.txt")
    groups = file.split("\n\n")
    seeds = groups[0].split(": ")[1].splitWhitespace()

  for dataString in groups[1..^1]:
    translations.add(generateTranslation(dataString))

  var locations = seeds.map do (seed: string) -> int:
    var value = parseInt(seed)

    for translation in translations:
      for info in translation:
        if value >= info.source and value < info.source + info.range:
          let diff = value - info.source
          value = info.destination + diff
          break
    value

  
  locations.sort()
  echo locations[0]

echo "========= Part One ==========="
partOne()

# ============================

proc translate(seed: SeedRange, translation: seq[TranslationRange]): seq[SeedRange] = 
  var 
    ranges: seq[SeedRange] = @[seed]
    i = 0
  
  while i < ranges.len():
    for tRange in translation:
      if ranges[i].start >= tRange.source and ranges[i].start < tRange.source + tRange.range:
        let 
          diffToStart = ranges[i].start - tRange.source
          diffToEnd = tRange.range - diffToStart
        
        if diffToEnd < ranges[i].length:
          let splitRange = (start: ranges[i].start + diffToEnd, length: ranges[i].length - diffToEnd)
          ranges.add(splitRange)
          ranges[i].length = diffToEnd
        
        ranges[i].start = tRange.destination + diffToStart
        break;
    i += 1
  ranges

proc partTwo() =
  var 
    translations: seq[seq[TranslationRange]] = @[]
    seedRanges: seq[SeedRange] = @[]
    locations: seq[SeedRange] = @[]
  let 
    file = readFile("2023/data/05/input.txt")
    groups = file.split("\n\n")
    seeds = groups[0].split(": ")[1].splitWhitespace()

  block seedParsing:
    var i = 0
    while i < seeds.len():
      seedRanges.add((start: parseInt(seeds[i]), length: parseInt(seeds[i + 1])))
      i += 2

  for dataString in groups[1..^1]:
    translations.add(generateTranslation(dataString))
  
  for seed in seedRanges:
    var translated = seed.translate(translations[0])

    for translation in translations[1..^1]:
      # Making a copy to maintain memory safety
      let translation = translation
      translated = foldl(translated.map(t => t.translate(translation)), a.concat(b), newSeq[SeedRange]())
    locations.add(translated)

  locations.sort()
  echo locations[0]
  
echo "========= Part Two ==========="
partTwo()