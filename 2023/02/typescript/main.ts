import { readFileSync } from "fs";

const RED_KEY = "red";
const MAX_RED = 12;
const GREEN_KEY = "green";
const MAX_GREEN = 13;
const BLUE_KEY = "blue";
const MAX_BLUE = 14;

const sumOfIds = readFileSync("../input.txt")
  .toString()
  .split("\n")
  .map((line) => {
    const [game, sets] = line.split(": ");
    const [_, gameId] = game.split(" ");
    const tabulation = {};

    sets.split("; ").map((set) => {
      set.split(", ").map((pairs) => {
        const [count, color] = pairs.split(" ");
        tabulation[color] = Math.max(tabulation[color] ?? 0, parseInt(count));
      });
    });

    return { ID: parseInt(gameId), tabulation };
  })
  .filter(({ tabulation }) => {
    return (
      (tabulation[RED_KEY] ?? 0) <= MAX_RED &&
      (tabulation[GREEN_KEY] ?? 0) <= MAX_GREEN &&
      (tabulation[BLUE_KEY] ?? 0) <= MAX_BLUE
    );
  })
  .map(({ ID }) => ID)
  .reduce((acc, id) => acc + id, 0);

console.log(sumOfIds);
