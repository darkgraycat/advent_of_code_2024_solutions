const ws = require('ws');
const readline = require('node:readline');

const sleep = (ms = 0) => new Promise(res => setTimeout(res, ms));

const FPS = 600;
const RENDER_DELAY = 1000 / FPS;
const LOOP_DELAY = 3000;
const STATE_RX = /!(?<state>[A-Za-z]+) ?(?<data>\w+)?/;

const STATES = {
  MAIN: 'MAIN',
  SIMULATION: 'SIMULATION',
  LOOP: 'LOOP',
};

const wss = new ws.Server({ port: 8080 });

(async () => {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false,
  });

  const broadcast = data => {
    wss.clients.forEach(client =>
      client.readyState === ws.OPEN &&
      client.send(JSON.stringify(data))
    );
  };

  let currentState;
  let loopCount;

  const frames = {
    [STATES.MAIN]: [],
    [STATES.SIMULATION]: [],
    [STATES.LOOP]: [],
  };

  for await (const line of rl) {
    if (line[0] != '!') {
      frames[currentState].push(line);
      continue;
    }

    const { state, data } = line.match(STATE_RX)?.groups || {};

    currentState = state;

    // console.log(`CMD: ${state}`);

    if (state == 'LOOP') {
      console.log(`Total loops: ${data}`);
      loopCount = data;
      await sleep(LOOP_DELAY);
    }

    // render
    // console.log(
    //   '----------MAIN----------\n',
    //   frames[STATES.MAIN].join('\n'),
    //   '------SIMULATION--------\n',
    //   frames[STATES.SIMULATION].join('\n'),
    // );

    broadcast({ frames, currentState, loopCount });

    frames[state].length = 0;
    await sleep(RENDER_DELAY);
  }


})();
