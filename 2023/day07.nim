import std/strutils
import std/sequtils
import std/sugar
import std/algorithm
import std/tables
import std/sets

type
  Hand = tuple[cards: string, bid: int]

# Idea:
# Assign values to hand strength, so.
# High Card: 0
# One Pair: 1
# Two Pair: 2
# Three of a kind: 3
# Full House: 4
# Four of a Kind: 5
# Five of a Kind: 6
# Then, we can use this in a sorting algorithm, along with 
# the index based sorting for equal strenght hands.

proc strength(c: char): int =
  if c == 'J':
    result = -1
  elif c == '2':
    result = 0
  elif c == '3':
    result = 1
  elif c == '4':
    result = 2
  elif c == '5':
    result = 3
  elif c == '6':
    result = 4
  elif c == '7':
    result = 5
  elif c == '8':
    result = 6
  elif c == '9':
    result = 7
  elif c == 'T':
    result = 8
  elif c == 'J':
    result = 9
  elif c == 'Q':
    result = 10
  elif c == 'K':
    result = 11
  elif c == 'A':
    result = 12

proc strength(hand: Hand): int = 
  let 
    cards = toCountTable(hand.cards)
    countSet = toHashSet(toSeq(cards.values))
    tableLength = cards.len()
  #echo hand.cards, " || ", cards.len()

  if countSet == toHashSet([1]): # High Card
    result = 0
  elif tableLength == 4 and countSet == toHashSet([1, 2]): # One Pair
    result = 1
  elif tableLength == 3 and countSet == toHashSet([1, 2]): # Two Pair
    result = 2
  elif tableLength == 3 and countSet == toHashSet([1, 3]): #Three of a Kind
    result = 3
  elif tableLength == 2 and countSet == toHashSet([2, 3]): # Full House
    result = 4
  elif tableLength == 2 and countSet == toHashSet([1, 4]): # Four of a Kind
    result = 5
  elif tableLength == 1:
    result = 6

proc partOne() =
  let
    file = readFile("2023/data/07/input.txt")
    lines = file.split("\n")

  var 
    hands: seq[Hand] = lines.map(line => (cards: line.split(" ")[0], bid: parseInt(line.split(" ")[1])))
    finalNumber = 0
  
  hands.sort do (x, y: Hand) -> int:
    result = cmp(x.strength, y.strength)
    
    if result == 0:
      for index, c in x.cards:
        result = cmp(x.cards[index].strength, y.cards[index].strength)
        if result != 0:
          break

  for index, hand in hands:
    finalNumber += hand.bid * (index + 1)

  echo finalNumber


partOne()
echo "======="

proc strengthWithJokers(hand: Hand): int =
  let 
    cards = toCountTable(hand.cards)
    countSet = toHashSet(toSeq(cards.values))
    tableLength = cards.len()

  if countSet == toHashSet([1]): # High Card
    if cards.hasKey('J'):
      result = 1 # Makes a One Pair
    else:
      result = 0 # High Card
  elif tableLength == 4 and countSet == toHashSet([1, 2]): # One Pair
    if cards.hasKey('J'):
      result = 3 # Makes a Three of a Kind, regardless of J == 1 or J == 2
    else:
      result = 1 # One Pair
  elif tableLength == 3 and countSet == toHashSet([1, 2]): # Two Pair
    if cards.hasKey('J') and cards['J'] == 1:
      result = 4 # Make a Full House
    elif cards.hasKey('J') and cards['J'] == 2:
      result = 5 # Make a Four of a Kind
    else:
      result = 2 # Two Pair
  elif tableLength == 3 and countSet == toHashSet([1, 3]): #Three of a Kind
    if cards.hasKey('J'):
      result = 5 # Makes a Four of a Kind, regardless of J == 1 or J == 3
    else:
      result = 3 # Three of a Kind
  elif tableLength == 2 and countSet == toHashSet([2, 3]): # Full House
    if cards.hasKey('J'):
      result = 6 # Makes a Five of a Kind, regardless of J == 2 or J == 3
    else:
      result = 4 # Full House
  elif tableLength == 2 and countSet == toHashSet([1, 4]): # Four of a Kind
    if cards.hasKey('J'):
      result = 6 # Makes a Five of a Kind, regardless of J == 1 or J == 4
    else:
      result = 5 # Four of a Kind
  elif tableLength == 1:
    result = 6
  
proc partTwo() =
  let
    file = readFile("2023/data/07/input.txt")
    lines = file.split("\n")

  var 
    hands: seq[Hand] = lines.map(line => (cards: line.split(" ")[0], bid: parseInt(line.split(" ")[1])))
    finalNumber = 0
  
  hands.sort do (x, y: Hand) -> int:
    result = cmp(x.strengthWithJokers, y.strengthWithJokers)
    
    if result == 0:
      for index, c in x.cards:
        result = cmp(x.cards[index].strength, y.cards[index].strength)
        if result != 0:
          break

  for index, hand in hands:
    finalNumber += hand.bid * (index + 1)

  echo finalNumber
  
partTwo()