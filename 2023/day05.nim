import std/strutils
import std/sequtils
import std/algorithm
import std/sugar

proc partOne() =
  var 
    translations: seq[seq[tuple[source: int, destination: int, range: int]]] = @[]
  let 
    file = readFile("2023/data/05/input.txt")
    groups = file.split("\n\n")
    seeds = groups[0].split(": ")[1].splitWhitespace()

  for dataString in groups[1..^1]:
    # construct a table.
    var translation: seq[tuple[source: int, destination: int, range: int]] = @[]
    let
      data = dataString.split(":\n")[1].split("\n")
    
    for row in data:
      let
        rowData = row.splitWhitespace()
        destination = parseInt(rowData[0])
        source = parseInt(rowData[1])
        range = parseInt(rowData[2])
      translation.add((source: source, destination: destination, range: range))
    translations.add(translation)

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

type
  SeedRange = tuple[start: int, length: int]
  TranslationRange = tuple[source: int, destination: int, range: int]

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

  block translationParsing:
    for dataString in groups[1..^1]:
      # construct a table.
      var translation: seq[TranslationRange] = @[]
      let
        data = dataString.split(":\n")[1].split("\n")
      
      for row in data:
        let
          rowData = row.splitWhitespace()
          destination = parseInt(rowData[0])
          source = parseInt(rowData[1])
          range = parseInt(rowData[2])
        translation.add((source: source, destination: destination, range: range))
      translations.add(translation)
  
  block seedRangeIterating:
    for seed in seedRanges:
      # Unpacked the flat_mapping because I'm a noob at nim. May circle back to this.
      var
        soils = seed.translate(translations[0])
        fertilizers = foldl(soils.map(s => s.translate(translations[1])), a.concat(b), newSeq[SeedRange]())
        waters = foldl(fertilizers.map(f => f.translate(translations[2])), a.concat(b), newSeq[SeedRange]())
        lights = foldl(waters.map(w => w.translate(translations[3])), a.concat(b), newSeq[SeedRange]())
        temps = foldl(lights.map(l => l.translate(translations[4])), a.concat(b), newSeq[SeedRange]())
        humidity = foldl(temps.map(t => t.translate(translations[5])), a.concat(b), newSeq[SeedRange]())
        locs = foldl(humidity.map(h => h.translate(translations[6])), a.concat(b), newSeq[SeedRange]())

      locations.add(locs)

  locations.sort()
  echo locations[0]
  
echo "========= Part Two ==========="
partTwo()