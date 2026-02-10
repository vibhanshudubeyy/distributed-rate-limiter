const URL = "http://localhost:3000";

async function sendRequests() {
  const requests = [];
  let okCount = 0;
  let limitedCount = 0;

  for (let i = 0; i < 1000; i++) {
    requests.push(
      fetch(URL)
        .then(res => {
          if (res.status === 200) {
            okCount++;
          } else if (res.status === 429) {
            limitedCount++;
          }
        })
        .catch(err => {
          console.error("Network error", err);
        })
    );
  }

  await Promise.all(requests);

  console.log("✅ Completed");
  console.log(`200 OK: ${okCount}`);
  console.log(`429 Too Many Requests: ${limitedCount}`);
}

sendRequests();
