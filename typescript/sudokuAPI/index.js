import fetch from "node-fetch";

async function getSudoku() {
  const data = await fetch("https://sudoku-api.vercel.app/api/dosuku");
  const res = await data.json();
  const board = res.newboard.grids[0].value;
  const solution = res.newboard.grids[0].solution;
  console.log(JSON.stringify(board));
  console.log(JSON.stringify(solution));
}

getSudoku();
