export const emptyGrid = (gridSize) => {
  const grid = [];
  for (let x = 0; x < gridSize[0]; x++) {
      grid.push(new Array(gridSize[1]).fill(0));
  }
  return grid;
}

export const inBounds = (coord, grid) => {
  const x = 0 <= coord[0] && coord[0] < grid.length;
  if (!x) return false;
  const y = 0 <= coord[1] && coord[1] < grid[coord[0]].length;
  return x && y;
};

export const wrap = (coord, grid) => {
  const gridSize = [grid.length, grid[0].length];
  while (coord[0] >= gridSize[0]) {
    coord[0] -= gridSize[0];
  }
  while (coord[0] < 0) {
    coord[0] += gridSize[0];
  }
  while (coord[1] >= gridSize[1]) {
    coord[1] -= gridSize[1];
  }
  while (coord[1] < 0) {
    coord[1] += gridSize[1];
  }
  return coord;
}

export const move = (coord, dir) => {
  return [coord[0] + dir[0], coord[1] + dir[1]];
};

export const lookup = (coord, grid) => {
  if (!inBounds(coord, grid)) return null;
  return grid[coord[0]][coord[1]];
};

export const difference = (c1, c2) => {
  const out = [];
  for (let i = 0; i < c1.length; i++) {
    out.push(c1[i] - c2[i]);
  }
  return out;
};

export const multiply = (coord, val) => {
  return coord.map(i => i*val);
}

export const vectorDistance = (vec1, vec2) => {
  if (vec1.length !== vec2.length) {
      throw new Error("Vectors must be of the same length.");
  }
  return Math.sqrt(vec1.reduce((sum, val, index) => sum + Math.pow(val - vec2[index], 2), 0));
};

export const divisible = (vec1, vec2) => {
  if (vec1.length !== vec2.length) return false;
  return !vec1.some((v, i) => v % vec2[i] !== 0 );
}

export const divide = (vec1, vec2) => {
  // do not use this unless you know the vectors are divisible
  return vec1[0] / vec2[0];
}

export const vectorMagnitude = (vec) => {
  return Math.sqrt(vec.reduce((sum, val) => sum + Math.pow(val, 2), 0));
};

export const determinant = (vec1, vec2) => {
  // assumes vectors are 2d for now.
  return (vec1[0] * vec2[1]) - (vec1[1] * vec2[0])
}

export const dirs = (diag=true) => {
  if (diag) return [
    [0, 1],
    [1, 0],
    [1, 1],
    [0, -1],
    [-1, 0],
    [-1, -1],
    [1, -1],
    [-1, 1],
  ];
  else return [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0],
  ];
}