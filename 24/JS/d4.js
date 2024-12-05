import { getInputs } from "./lib/inputs.js";

const i = getInputs(4).toString();
const testInput = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`;

const grid = i.split("\n").map((line) => [...line]);
const testGrid = testInput.split("\n").map((line) => [...line]);

const matchString = "XMAS";
const dirs = [
  [0, 1],
  [1, 0],
  [1, 1],
  [0, -1],
  [-1, 0],
  [-1, -1],
  [1, -1],
  [-1, 1],
];

const inBounds = (coord, grid) => {
  const x = 0 <= coord[0] && coord[0] < grid.length;
  const y = 0 <= coord[1] && coord[1] < grid[0].length;
  return x && y;
};

const move = (coord, dir) => {
  return [coord[0] + dir[0], coord[1] + dir[1]];
};

const lookup = (coord, grid) => {
  if (!inBounds(coord, grid)) return null;
  return grid[coord[0]][coord[1]];
};

const search = (grid, start, string, dir = null) => {
  if (string.length === 0) {
    return 1;
  }
  const x = start[0];
  const y = start[1];
  if (lookup([x, y], grid) === string[0]) {
    if (!dir) {
      const results = dirs
        .filter((dir) => {
          return lookup(move([x, y], dir), grid) === string[1];
        })
        .map((dir) => {
          return search(
            grid,
            move([x, y], dir),
            string.slice(1, string.length),
            dir
          );
        });
      return results.length ? results.reduce((c, i) => c + i) : 0;
    } else if (string.length === 1) {
      return 1;
    } else {
      return search(
        grid,
        move([x, y], dir),
        string.slice(1, string.length),
        dir
      );
    }
  } else return 0;
};

const checkGrid = (grid, string) => {
  let total = 0;
  for (let x = 0; x < grid.length; x++) {
    for (let y = 0; y < grid[0].length; y++) {
      if (lookup([x, y], grid) === string[0]) {
        total += search(grid, [x, y], string);
      }
    }
  }
  return total;
};

const testTotal = checkGrid(testGrid, matchString);
const total = checkGrid(grid, matchString);

const masGrid = [
  [
    ["M", ".", "M"],
    [".", "A", "."],
    ["S", ".", "S"],
  ],
  [
    ["S", ".", "M"],
    [".", "A", "."],
    ["S", ".", "M"],
  ],
  [
    ["S", ".", "S"],
    [".", "A", "."],
    ["M", ".", "M"],
  ],
  [
    ["M", ".", "S"],
    [".", "A", "."],
    ["M", ".", "S"],
  ],
];

const zoneCompare = (grid, subGrid, start) => {
  for (let x = 0; x < subGrid.length; x++) {
    for (let y = 0; y < subGrid[0].length; y++) {
      const gX = x + start[0];
      const gY = y + start[1];
      const subGridValue = lookup([x, y], subGrid);
      if (subGridValue === ".") continue;
      if (lookup([gX, gY], grid) !== subGridValue) {
        return 0;
      }
    }
  }
  return 1;
};

let testXmases = testGrid.map((row, x) => {
  if (x <= testGrid.length - 3) {
    const isMases = row.map((col, y) => {
      if (y <= testGrid[0].length - 3) {
        const val = lookup([x, y], testGrid);
        if (val === 'M' || val === 'S') {
            const isMas = masGrid.some(g => zoneCompare(testGrid, g, [x, y]));
            return isMas ? 1 : 0;
        }
      }
    });
    return  isMases.filter(m => !!m).length ? isMases.filter(m => !!m).reduce((c, i) => c + i) : 0;
  }
  return 0;
})
testXmases = testXmases.length ? testXmases.reduce((c, i) => c + i) : 0;
console.log(testXmases);

let xmases = grid.map((row, x) => {
    if (x <= grid.length - 3) {
      const isMases = row.map((col, y) => {
        if (y <= grid[0].length - 3) {
          const val = lookup([x, y], grid);
          if (val === 'M' || val === 'S') {
              const isMas = masGrid.some(g => zoneCompare(grid, g, [x, y]));
              return isMas ? 1 : 0;
          }
        }
      });
      return  isMases.filter(m => !!m).length ? isMases.filter(m => !!m).reduce((c, i) => c + i) : 0;
    }
    return 0;
  })
  xmases = xmases.length ? xmases.reduce((c, i) => c + i) : 0;
  console.log(xmases);