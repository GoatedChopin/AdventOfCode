export const inBounds = (coord, grid) => {
  const x = 0 <= coord[0] && coord[0] < grid.length;
  if (!x) return false;
  const y = 0 <= coord[1] && coord[1] < grid[coord[0]].length;
  return x && y;
};

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