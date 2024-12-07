import { getInputs } from "./lib/inputs.js";

const i = `....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...`;

const m = i.split('\n').map(x => x.split(''));
const inputs = getInputs(6).toString().split('\n').map(x => x.split(''));

const inBounds = (grid, coord) => {
  return coord[0] >= 0 && coord[0] < grid.length && coord[1] >= 0 && coord[1] < grid[0].length;
}

const move = (coord, dir) => {
  return [coord[0] + dir[0], coord[1] + dir[1]];
}

const deltas = [[-1, 0], [0, 1], [1, 0], [0, -1]];

const partOne = (m) => {
  let start = [0, 0]
  for (let x = 0; x < m.length; x++) {
    for (let y = 0; y < m[x].length; y++) {
      if (m[x][y] === '^') start = [x, y]
  }}
  const carat = [...start];
  let dir = deltas[0]
  while (inBounds(m, start)) {
    let next = move(start, dir)
    // console.log(next);
    while (m[next[0]]?.[next[1]] === '#') {
      const dirInd = deltas.indexOf(dir)
      dir = deltas[dirInd < 3 ? dirInd + 1 : 0]
      next = move(start, dir)
    }
    m[start[0]][start[1]] = 'X';
    start = next;
  }
  console.log(m.map(x => x.join('')).join('\n'))
  return { m, start: carat }
}

console.log(partOne(m).m.map(x => x.map(y => y === 'X' ? 1 : 0).reduce((c, i) => c + i)).reduce((c, i) => c + i));
console.log(partOne(inputs).m.map(x => x.map(y => y === 'X' ? 1 : 0).reduce((c, i) => c + i)).reduce((c, i) => c + i));

const partTwoTestInputs = partOne(m);
const partTwoInputs = partOne(inputs);

const isLoop = (m, start) => {
    const visited = new Set();
    let dir = deltas[0]
    while (inBounds(m, start)) {
      let next = move(start, dir)
      while (m[next[0]]?.[next[1]] === '#') {
        const dirInd = deltas.indexOf(dir)
        dir = deltas[dirInd < 3 ? dirInd + 1 : 0]
        next = move(start, dir)
      }
      if (visited.has([next, dir])) {
        return true;
      }
      visited.add([next, dir]);
      start = next;
    }
    return false;
}

const partTwo = (args) => {
    const { m, start } = args;
    const xIndices = [];
    for (let x = 0; x < m.length; x++) {
        for (let y = 0; y < m[x].length; y++) {
            if (m[x][y] === 'X') xIndices.push([x, y]);
        }
    }
    let loops = 0;
    for (let i = 0; i < xIndices.length; i++) {
        const ind = xIndices[i];
        const alteredGrid = m.map((_, x) => {
            return _.map((__, y) => {
                if ([x, y].join('') == ind.join('')) {
                    return '#';
                } else return m[x][y]
            }) 
        })
        if (isLoop(alteredGrid, start)) {
            loops += 1;
        } else {
            console.log(`No loop for ${ind}`);
        }
    }
    return loops;
}

console.log(isLoop(m, [5, 4]));
console.log(partTwo(partOne(i.split('\n').map(x => x.split('')))));
// console.log(partTwo(partOne(getInputs(6).toString().split('\n').map(x => x.split('')))));