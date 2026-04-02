declare namespace wasm_bindgen {
    /* tslint:disable */
    /* eslint-disable */

    export function wasm32_browser_main(): Promise<void>;

}
declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly wasm32_browser_main: () => void;
    readonly main: (a: number, b: number) => number;
    readonly wasm_bindgen_768f608d6e2bdde0___convert__closures_____invoke___wasm_bindgen_768f608d6e2bdde0___JsValue__core_d9ade5aacaa0adab___result__Result_____wasm_bindgen_768f608d6e2bdde0___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_768f608d6e2bdde0___convert__closures_____invoke___bool__true_: (a: number, b: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
declare function wasm_bindgen (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
