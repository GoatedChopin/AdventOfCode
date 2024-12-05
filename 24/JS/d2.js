import { getInputs } from './lib/inputs.js';

const i = getInputs(2).toString();

// Using match() with global flag
const pattern = /your_pattern_here/g;
const matches = i.match(pattern);

// Or using matchAll() for more detailed results including capture groups
const matchesWithGroups = [...i.matchAll(/your_pattern_here/g)];

// Or using exec() in a loop
const regex = /your_pattern_here/g;
let match;
const allMatches = [];
while ((match = regex.exec(i)) !== null) {
    allMatches.push(match);
}