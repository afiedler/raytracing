import GithubIcon from "./GithubIcon";

export interface HeaderProps {
  state: string;
  onRaytraceClick: () => void;
  raytraceDisabled: boolean;
}
export default function Header(props: HeaderProps) {
  return (
    <nav className="bg-gray-600">
      <div className="px-4">
        <div className="flex items-center justify-between h-16">
          <div className="flex items-center">
            <div className="flex-shrink-0 text-white px-2 border-r-solid border-r-2 border-r-gray-300">
              Rust WASM Raytracer
            </div>
            <div className="flex-shrink-0 text-white px-2 border-r-solid border-r-2 border-r-gray-300 font-mono">
              {props.state}
            </div>

            <button
              className="text-gray-100 bg-gray-400 hover:text-white px-3 py-2 mx-2 rounded-md font-medium"
              onClick={props.onRaytraceClick}
              disabled={props.raytraceDisabled}
            >
              Raytrace!
            </button>
          </div>
          <div className="flex items-center">
            <div className="flex-shrink-0 text-white px-2 border-r-solid border-r-2 border-r-gray-300">
              <a href="https://andyfiedler.com">andyfiedler.com</a>
            </div>
            <div className="flex-shrink-0 text-white px-2">
              <a href="https://github.com/afiedler/raytracing">
                <GithubIcon className="text-white" />
              </a>
            </div>
          </div>
        </div>
      </div>
    </nav>
  );
}
