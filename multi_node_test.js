const URLS = [
  "http://localhost:3000",
  "http://localhost:3001",
  "http://localhost:3002",
];

const TOTAL = 1000;
const CONCURRENCY = 50;

let ok = 0, limited = 0;

async function worker(queue) {
  while (queue.length) {
    const i = queue.pop();
    const res = await fetch(URLS[i % URLS.length]);
    if (res.status === 200) ok++;
    if (res.status === 429) limited++;
  }
}

async function run() {
  const queue = Array.from({ length: TOTAL }, (_, i) => i);
  await Promise.all(
    Array.from({ length: CONCURRENCY }, () => worker(queue))
  );

  console.log({ ok, limited });
}

run();
