const fs = require('node:fs/promises');
const readline = require('node:readline/promises');
const path = require('node:path');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

const getch = () => rl.question('>');

const printSideBySide = (str1, str2) => {
    const lines1 = str1.split('\n');
    const lines2 = str2.split('\n');

    for (let i = 0; i < lines1.length; i++) {
        console.log(`${lines1[i]}  ${lines2[i]}`);
    }
}

const X = 0;
const Y = 1;

const DirectionXy = {
  '^': [0, -1],
  'v': [0, 1],
  '<': [-1, 0],
  '>': [1, 0],
};

(async () => {
  const input = await fs.readFile(path.resolve(__dirname, 'input.txt'), 'utf8');

  const guardIdx = input.search(/[<>v^]/);
  const scanline = input.indexOf('\n') + 1;

  const guard = {
    dir: input[guardIdx],
    pos: [
      guardIdx % scanline | 0,
      guardIdx / scanline | 0,
    ],
    visited: new Set(),
    isVisited() {
      const [x, y] = this.pos;
      return this.visited.has(`${x},${y},${this.dir}`);
    },
    markVisited() {
      const [x, y] = this.pos;
      this.visited.add(`${x},${y},${this.dir}`);
    },
    clone() {
      return { ...this, pos: [...this.pos], visited: new Set(this.visited) };
    },
    getNext() {
      const [dx, dy] = DirectionXy[this.dir];
      return [this.pos[X] + dx, this.pos[Y] + dy];
    },
    turnRight() {
      this.dir = {
        '^': '>',
        '>': 'v',
        'v': '<',
        '<': '^',
      }[this.dir];
    },
  };

  const state = {
    width: input.indexOf('\n'),
    height: input.split('\n').length - 1,
    blocks: new Set([...input.matchAll(/#/g)].map(m => {
      const x = m.index % scanline | 0;
      const y = m.index / scanline | 0;
      return `${x},${y}`;
    })),
    isBlock([x, y]) {
      return this.blocks.has(`${x},${y}`);
    },
    isOutOfBounds([x, y]) {
      return x < 0 || x >= this.width || y < 0 || y >= this.height;
    },
    print(guard) {
      const line = '.'.repeat(this.width) + '\n';
      const grid = line.repeat(this.height).trim().split('\n').map(line => line.split(''));

      this.blocks.forEach(bl => {
        const [x, y] = bl.split(',');
        grid[y][x] = '#';
      });

      guard.visited.forEach(vis => {
        const [x, y, dir] = vis.split(',');
        grid[y][x] = ['^','v'].includes(dir) ? '|' : '-';
      });

      const [x, y] = guard.pos;
      if (!this.isOutOfBounds([x, y])) {
        grid[y][x] = guard.dir;
      }

      return grid.map(line => line.join('')).join('\n');
    }
  };

  let maxIterations = 1000000;
  let loops = 0;

  const loopsSet = new Set();

  /* main loop */
  while (maxIterations--) {
    // console.log('-------------------------------------');

    const nextStep = guard.getNext();
    // done
    if (state.isOutOfBounds(nextStep)) {
      break;
    }
    // turn right and next iteration
    if (state.isBlock(nextStep)) {
      guard.turnRight();
      continue;
    }

    /* simulation loop */
    const sim = guard.clone();
    sim.turnRight(); // imitate block

    while (true) {
      // await getch();
      // printSideBySide(state.print(guard), state.print(sim));
      // console.log();

      const nextSimStep = sim.getNext();
      // exit simulation
      if (state.isOutOfBounds(nextSimStep)) {
        break;
      }
      // turn right and next iteration
      if (state.isBlock(nextSimStep)) {
        sim.turnRight();
        continue;
      }
      // loop found - exit simulation
      if (sim.isVisited()) {
        loops++;
        const [x, y] = nextStep;
        loopsSet.add(`${x},${y}`);
        break;
      }

      // make next step....
      sim.markVisited();
      sim.pos = nextSimStep;
    }

    // make next step....
    guard.markVisited();
    guard.pos = nextStep;
  }

  return loopsSet.size;
})()
  .then(console.log)
  .catch(console.error)
  .finally(process.exit);
