import { getInputs } from "./lib/inputs.js";

const i = getInputs(3).toString();

const mulRegex = /mul\(\d+,\d+\)/g;
const matches = [...i.matchAll(mulRegex)];

// Using eval (simple but less safe):
// const results = matches.map(match => {
//     return match[0];  // converts 'mul(2,3)' to '(2,3)' which evaluates to 3
// });

// Or a safer custom approach:
const evaluateMul = (mulStr) => {
  const [a, b] = mulStr
    .replace("mul(", "")
    .replace(")", "")
    .split(",")
    .map(Number);
  return a * b;
};

const saferResults = matches.map((match) => evaluateMul(match[0]));

console.log(saferResults.reduce((p, c) => p + c));

const mulMatches = [...i.matchAll(mulRegex)];
const doRegex = /do\(\)/g;
const doMatches = [...i.matchAll(doRegex)];
const dontRegex = /don\'t\(\)/g;
const dontMatches = [...i.matchAll(dontRegex)];

const combinedInstructions = [
  ...doMatches.map((d) => {
    return { ...d, isDo: true };
  }),
  ...dontMatches.map((d) => {
    return { ...d, isDo: false };
  }),
  ...mulMatches,
];

combinedInstructions.sort((a, b) => {
  return a.index - b.index;
});

const state = { doing: true, total: 0 };
const filteredInstructions = combinedInstructions.map((match) => {
  if (typeof match.isDo === "boolean") state.doing = match.isDo;
  else if (!!state.doing) {
    const result = evaluateMul(match[0]);
    state.total = state.total + result;
    return result;
  }
});
console.log(state);
