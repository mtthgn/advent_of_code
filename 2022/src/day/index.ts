import * as one from "./01.js"
import * as two from "./02.js"
import * as three from "./03.js"
import * as four from "./04.js"
import * as five from "./05.js"
import * as six from "./06.js"
import * as seven from "./07.js"
import * as eight from "./08.js"
import * as nine from "./09.js"
import * as ten from "./10.js"
import * as eleven from "./11.js"
import * as twelve from "./12.js"
import * as thirteen from "./13.js"
import * as fourteen from "./14.js"
import * as fifteen from "./15.js"
import * as sixteen from "./16.js"
import * as seventeen from "./17.js"
import * as eighteen from "./18.js"
import * as nineteen from "./19.js"
import * as twenty from "./20.js"
import * as twenty_one from "./21.js"
import * as twenty_two from "./22.js"
import * as twenty_three from "./23.js"
import * as twenty_four from "./24.js"
import * as twenty_five from "./25.js"

export type Day = {
  partOne: (data: string) => void
  partTwo: (data: string) => void
}

type DaysMap = {
  [key: string]: Day
}
const days: DaysMap = {
  one,
  two,
  three,
  four,
  five,
  six,
  seven,
  eight,
  nine,
  ten,
  eleven,
  twelve,
  thirteen,
  fourteen,
  fifteen,
  sixteen,
  seventeen,
  eighteen,
  nineteen,
  twenty,
  twenty_one,
  twenty_two,
  twenty_three,
  twenty_four,
  twenty_five,
}

export default days
