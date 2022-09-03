import { useCallback, useEffect, useRef, useState } from "react";
import GithubIcon from "./GithubIcon";
import { Header } from "./Header";
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
  const canvasContainerRef = useRef<HTMLDivElement>(null);

  useEffect(
    () =>
      whenWorkersAreReady((worker) => {
        setState({ type: "workers-ready", worker });
      }),
    []
  );

  useEffect(() => {
    const r = new ResizeObserver((entries) => {
      for (const entry of entries) {
        if (entry.target === canvasContainerRef.current) {
          canvasRef.current!.style.width = `${entry.contentRect.width}px`;
          canvasRef.current!.style.height = `${
            (entry.contentRect.width * 800) / 1200
          }px`;
        }
      }
    });

    r.observe(canvasContainerRef.current!);
    return () => {
      r.disconnect();
    };
  });

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
      <div ref={canvasContainerRef} className="w-full bg-slate-600">
        <canvas width={1200} height={800} ref={canvasRef} />
      </div>

      <div className="flex items-center w-full mt-6 ml-1 mr-1">
        <div className="flex-shrink-0 text-gray-200 px-4 border-r-solid border-r-2 border-r-gray-300">
          <a href="https://andyfiedler.com">andyfiedler.com</a>
        </div>
        <div className="flex-shrink-0 text-gray-200 px-4">
          <a href="https://github.com/afiedler/raytracing">
            <GithubIcon className="text-gray-200" />
          </a>
        </div>
      </div>
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
