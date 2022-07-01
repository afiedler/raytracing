import { useCallback, useEffect, useRef, useState } from "react";
import Header from "./Header";
import { whenWorkersAreReady } from "./whenWorkersAreReady";

type State =
  | { type: "awaiting-workers" }
  | { type: "workers-ready"; worker: Worker }
  | { type: "raytracing" }
  | { type: "finished" };

const isSafari =
  navigator.userAgent.includes("Safari") &&
  !navigator.userAgent.includes("Chrome");

function App() {
  const [state, setState] = useState<State>({ type: "awaiting-workers" });
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(
    () =>
      whenWorkersAreReady((worker) => {
        setState({ type: "workers-ready", worker });
      }),
    []
  );

  const onButtonClick = useCallback(() => {
    if (state.type !== "workers-ready") return;

    state.worker.addEventListener("message", (event) => {
      if (event.data.type === "progress") {
        const imgData = new ImageData(event.data.line, 1200, 1);
        canvasRef.current
          ?.getContext("2d")
          ?.putImageData(imgData, 0, event.data.lineNumber);
      } else if (event.data.type === "finished") {
        setState({ type: "finished" });
      }
    });

    state.worker.postMessage({ type: "raytrace" });

    setState({ type: "raytracing" });
  }, [state, setState]);

  return (
    <>
      {isSafari && <SafariWarning />}
      <Header
        state={state.type}
        onRaytraceClick={onButtonClick}
        raytraceDisabled={
          !(state.type === "workers-ready" || state.type === "finished")
        }
      />
      <canvas width={1200} height={800} ref={canvasRef} />
    </>
  );
}

const SafariWarning = () => (
  <div className="bg-red-700 text-white px-6 py-1">
    This demo does not work on Safari because{" "}
    <a
      href="https://bugs.webkit.org/show_bug.cgi?id=22723"
      className="underline"
    >
      nested workers are not supported
    </a>
    .
  </div>
);

export default App;
