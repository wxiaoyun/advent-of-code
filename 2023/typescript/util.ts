import { readFileSync } from "fs";

export const getInputForDay = (day: number): string => {
  return readFileSync(
    `../questions/${String(day).padStart(2, "0")}.txt`,
  ).toString();
};
