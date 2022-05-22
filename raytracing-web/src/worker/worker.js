async function start() {
  let raylib = await import("raylib-web");
  await raylib.default();
  raylib.run();

  self.addEventListener("message", (event) => {
    if (event.data.type === "raytrace") {
      raylib.raytrace();
    }
  });

  self.postMessage({ type: "ready" });
}

start();
