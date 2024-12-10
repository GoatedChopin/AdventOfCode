import { getInputs } from "./lib/inputs.js";

const i = getInputs(5).toString();

const testInput = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`;

const parseInstructions = (instructions, test = false) => {
  const parts = instructions.replaceAll("\r", "").split("\n\n");
  const constraints = {};
  const forward = {};
  parts[0].split("\n").map((c) => {
    const inds = c.split("|").map((i) => +i);
    if (!constraints[inds[1]]) constraints[inds[1]] = new Set();
    constraints[inds[1]].add(inds[0]);
    if (!forward[inds[0]]) forward[inds[0]] = new Set();
    forward[inds[0]].add(inds[1]);
  });
  const pages = parts[1]
    .split("\n")
    .map((p) => p.split(",").map((i) => +i));
  return { constraints, forward, pages };
};

const testInstructions = parseInstructions(testInput);
const instructions = parseInstructions(i);

const difference = (setA, setB) => {
  const result = new Set(setA);
  if (typeof setB === "undefined") {
    console.log("Huh?");
  }
  if (!setB.size) return result;
  for (let item of setB) {
    result.delete(item); // Remove elements from result that are also in setB
  }
  return result;
};

const union = (setA, setB) => {
  const result = new Set();
  setA.forEach((i) => {
    if (setB.has(i)) result.add(i);
  });
  return result;
};

// const isCorrect = (constraints, pages) => {
//     const visited = new Set();
//     for (const i in pages) {
//         const p = pages[i]
//         const missingPages = difference(constraints[p] || new Set(), visited);
//         if (missingPages.size > 0) return false;
//         visited.add(p);
//     }
//     return true;
// }

const isCorrect = (constraints, pages) => {
  const visited = [];
  const collisions = new Set();
  for (const i in pages) {
    const p = pages[i];
    if (collisions.has(p)) {
      return false;
    }
    collisions.add(...(constraints[p] || []));
    visited.push(p);
  }
  return true;
};

const partOne = (inst) => {
  const { constraints, forward, pages } = inst;
  const middleElement = (l) => {
    return l[Math.floor(l.length / 2)];
  };
  const result = pages.map((p) => {
    if (isCorrect(constraints, p)) return middleElement(p);
    return 0;
  });
  return result.reduce((c, i) => c + i); 
};

console.log(partOne(testInstructions));
console.log(partOne(instructions));
