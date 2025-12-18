const input = await Bun.file('input.txt').text();

// For part1, use MAX_EPOCHS = 1
const MAX_EPOCHS = Infinity;

const width = input.indexOf('\n');
const grid = Array.from(input);

const countOcurrences = (arr: any[], what: any[]) => arr.reduce((acc, curr) => (acc + (what.includes(curr) ? 1 : 0)), 0);

let count = 0;

for (let e = 0; e < MAX_EPOCHS; e++) {
    let initialCount = count;

    for (let i = 0; i < grid.length; i++) {
        const curr = grid[i];
        if (curr === '\n' || curr === '.' || curr === 'x') continue;

        const occ = countOcurrences([
            grid[i-width-2], grid[i-width-1], grid[i-width],
            grid[i-1],     /*grid[i],*/       grid[i+1],
            grid[i+width],   grid[i+width+1], grid[i+width+2]
        ], ['@', 'x']);

        if (occ < 4) {
            grid[i] = 'x';
            count++;
        }
    }

    for (let i = 0; i < grid.length; i++) {
        if (grid[i] === 'x') grid[i] = '.';
    }

    if (initialCount === count) break;
}

console.log(grid.join(''))
console.log({count})

export {};
