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

const parseInstructions = (instructions, test=false) => {
  const parts = test ? instructions.split("\n\n") : instructions.split("\r\n\r\n");
  const constraints = {};
  parts[0].split("\n").map((c) => {
    const inds = c.split("|");
    if (!constraints[inds[1]]) constraints[inds[1]] = new Set();
    constraints[inds[1]].add(inds[0]);
  });
  const pages = test ? parts[1].split("\n").map((p) => p.split(",")) : parts[1].split("\r\n").map((p) => p.split(","));
  return { constraints, pages };
};

const testInstructions = parseInstructions(testInput, true);
const instructions = parseInstructions(i);

const isCorrect = (constraints, pages) => {
    const visited = new Set();
    pages.forEach(p => {
        const missingPages = visited.difference(constraints[p]);
        if (missingPages.length > 0) return false;
        visited.add(p);
    })
    return true;
}

const partOne = (inst) => {
    const { constraints, pages } = inst;
    const middleElement = (l) => {
        return l[Math.floor(l.length / 2)]
    }
    const result = pages.map(p => {
        if (isCorrect(constraints, p)) return middleElement(p);
        return 0;
    }).reduce((c, i) => c + i);
}

console.log(partOne(testInstructions));
console.log(partOne(instructions));