import fs from "node:fs";
import { URL } from "node:url";

const content = await fs.promises.readFile(
  new URL("./input.txt", import.meta.url).pathname,
  "utf-8"
);

solve(content);

/**
 * @param {string} content
 */
function solve(content) {
  const input = parse(content);
  const windows = getWindows(input, 3);
  const increases = countIncreases(windows);
  console.log(increases);
}

/**
 * @param {string} content
 * @returns {number[]}
 */
function parse(content) {
  return content
    .split("\n")
    .slice(0, -1)
    .map((x) => parseInt(x));
}

/**
 * @param {number[]} input
 * @param {number} size
 * @returns {number[]}
 */
function getWindows(input, size) {
  return input.slice(0, input.length + 1 - size).map((_, i) => {
    let sum = 0;
    for (let j = i; j < i + size; j++) {
      sum += input[j];
    }
    return sum;
  });
}

/**
 * @param {number[]} values
 * @returns {number}
 */
function countIncreases(values) {
  let previous = Infinity;
  let sum = 0;
  for (const item of values) {
    if (item > previous) {
      sum++;
    }
    previous = item;
  }
  return sum;
}
