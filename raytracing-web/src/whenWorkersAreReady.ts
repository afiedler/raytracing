let areWorkersReady = false;
let workersReadyCallbacks: any[] = [];
let rootWorker: Worker | null = null;

export function whenWorkersAreReady(
  callback: (rootWorker: Worker) => void
): () => void {
  if (areWorkersReady && rootWorker) {
    const worker = rootWorker;
    setTimeout(() => callback(worker));
    return () => {};
  }

  workersReadyCallbacks.push(callback);
  return () => {
    const index = workersReadyCallbacks.indexOf(callback);
    if (index > -1) {
      workersReadyCallbacks.slice(index, 1);
    }
  };
}

(async () => {
  rootWorker = new Worker(new URL("./worker/worker.js", import.meta.url), {
    type: "module",
  });

  rootWorker.addEventListener("message", (event) => {
    if (event.target !== rootWorker) return;
    if (event.data.type === "ready") {
      areWorkersReady = true;
      workersReadyCallbacks.forEach((callback) => {
        setTimeout(() => callback(rootWorker));
      });
    }
  });
})();
