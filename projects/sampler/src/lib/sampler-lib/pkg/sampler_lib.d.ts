/* tslint:disable */
/* eslint-disable */
export function main(): void;
export class SamplingProcessor {
  free(): void;
  constructor();
  /**
   * @param {string} timestamp
   * @param {string} measurement_type
   * @param {number} value
   */
  add_measurement(timestamp: string, measurement_type: string, value: number): void;
  /**
   * @returns {string}
   */
  process_measurements(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_samplingprocessor_free: (a: number, b: number) => void;
  readonly samplingprocessor_new: () => number;
  readonly samplingprocessor_add_measurement: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly samplingprocessor_process_measurements: (a: number, b: number) => void;
  readonly main: () => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
