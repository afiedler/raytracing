import { Popover, Transition } from "@headlessui/react";
import { Fragment, useState } from "react";

export interface HeaderProps {
  state: string;
  onRaytraceClick: () => void;
  raytraceDisabled: boolean;
}

export interface RenderOptions {
  width: number;
  height: number;
}

export function Header(props: HeaderProps) {
  const [options, setOptions] = useState<RenderOptions>({
    width: 1200,
    height: 800,
  });

  return (
    <Popover className="relative z-0 w-full">
      {({ open }) => (
        <>
          <div className="relative z-10 bg-white shadow w-full flex justify-between">
            <div className="flex max-w-7xl px-4 py-2 sm:px-6 lg:px-8">
              <span className="inline-flex items-center bg-white px-4 py-2 text-sm font-medium text-gray-700">
                Rust WASM Raytracer
              </span>
              <span className="ml-4 isolate inline-flex rounded-md shadow-sm">
                <span className="relative inline-flex items-center rounded-l-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700">
                  {props.state}
                </span>

                {/* <Popover.Button
                  className={classNames(
                    open ? "text-gray-900" : "text-gray-700",
                    "relative -ml-px inline-flex items-center border border-gray-300 bg-white px-4 py-2 text-sm font-medium hover:bg-gray-50 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
                  )}
                >
                  <span>Options</span>
                  <ChevronDownIcon
                    className={classNames(
                      open ? "text-gray-900" : "text-gray-700",
                      "ml-2 h-5 w-5 group-hover:text-gray-500"
                    )}
                    aria-hidden="true"
                  />
                </Popover.Button> */}

                <button
                  type="button"
                  className="relative -ml-px inline-flex items-center rounded-r-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
                  onClick={props.onRaytraceClick}
                  disabled={props.raytraceDisabled}
                >
                  Raytrace
                </button>
              </span>
            </div>
          </div>

          <Transition
            as={Fragment}
            enter="transition ease-out duration-200"
            enterFrom="opacity-0 -translate-y-1"
            enterTo="opacity-100 translate-y-0"
            leave="transition ease-in duration-150"
            leaveFrom="opacity-100 translate-y-0"
            leaveTo="opacity-0 -translate-y-1"
          >
            <Popover.Panel className="absolute inset-x-0 z-10 transform shadow-lg">
              <div className="absolute inset-0 flex" aria-hidden="true">
                <div className="w-1/2 bg-white" />
                <div className="w-1/2 bg-gray-50" />
              </div>
              <div className="relative mx-auto grid max-w-7xl grid-cols-1">
                <nav
                  className="grid gap-y-10 bg-white px-4 py-4"
                  aria-labelledby="options-heading"
                >
                  <h2 id="options-heading" className="sr-only">
                    Options menu
                  </h2>
                  <form className="space-y-8 divide-y divide-gray-200">
                    <div className="space-y-8 divide-y divide-gray-200 sm:space-y-5">
                      <div className="space-y-6 sm:space-y-5">
                        <div>
                          <h3 className="text-lg font-medium leading-6 text-gray-900">
                            Rendering Options
                          </h3>
                          <p className="mt-1 max-w-2xl text-sm text-gray-500">
                            {/*help text here*/}
                          </p>
                        </div>

                        <div className="space-y-6 sm:space-y-5">
                          <div className="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5">
                            <label
                              htmlFor="username"
                              className="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"
                            >
                              Image Size
                            </label>
                            <div className="mt-1 sm:col-span-2 sm:mt-0">
                              <div className="flex -space-x-px">
                                <div className="w-1/2 min-w-0 flex-1">
                                  <label
                                    htmlFor="image-width"
                                    className="sr-only"
                                  >
                                    Image width
                                  </label>
                                  <input
                                    type="text"
                                    name="image-width"
                                    id="image-width"
                                    className="relative block w-full rounded-l-md border-gray-300 bg-transparent focus:z-10 focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                    placeholder="Width (e.g., 1200)"
                                    value={options.width}
                                  />
                                </div>
                                <div className="min-w-0 flex-1">
                                  <label
                                    htmlFor="image-height"
                                    className="sr-only"
                                  >
                                    Image height
                                  </label>
                                  <input
                                    type="text"
                                    name="image-height"
                                    id="image-height"
                                    className="relative block w-full rounded-r-md border-gray-300 bg-transparent focus:z-10 focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                    placeholder="Height (e.g., 800)"
                                    value={options.height}
                                  />
                                </div>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </form>
                </nav>
              </div>
            </Popover.Panel>
          </Transition>
        </>
      )}
    </Popover>
  );
}
