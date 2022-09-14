/* tslint:disable */
/* eslint-disable */
/**
*/
export function wasm_startup(): void;
/**
*/
export function play(): void;
/**
*/
export enum ScrollDirection {
  Up,
  Down,
}
/**
*/
export class Engine {
  free(): void;
/**
*/
  constructor();
/**
* @param {SettingsMerge} partial_settings
* @returns {boolean}
*/
  modify_settings(partial_settings: SettingsMerge): boolean;
/**
* @returns {Settings}
*/
  toJSON(): Settings;
}
/**
* Stores RRR settings to start charts with.
*/
export class Settings {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
/**
*/
  judge_position: number;
/**
*/
  lane_gap: number;
/**
*/
  muted: boolean;
/**
*/
  scroll_direction: number;
/**
*/
  scroll_speed: number;
}
/**
*/
export class SettingsMerge {
  free(): void;
/**
*/
  constructor();
/**
*/
  judge_position?: number;
/**
*/
  lane_gap?: number;
/**
*/
  muted?: boolean;
/**
*/
  scroll_direction?: number;
/**
*/
  scroll_speed?: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_get_settingsmerge_scroll_speed: (a: number) => number;
  readonly __wbg_set_settingsmerge_scroll_speed: (a: number, b: number) => void;
  readonly __wbg_get_settingsmerge_judge_position: (a: number, b: number) => void;
  readonly __wbg_set_settingsmerge_judge_position: (a: number, b: number, c: number) => void;
  readonly __wbg_get_settingsmerge_scroll_direction: (a: number) => number;
  readonly __wbg_set_settingsmerge_scroll_direction: (a: number, b: number) => void;
  readonly __wbg_get_settingsmerge_lane_gap: (a: number) => number;
  readonly __wbg_set_settingsmerge_lane_gap: (a: number, b: number) => void;
  readonly __wbg_get_settingsmerge_muted: (a: number) => number;
  readonly __wbg_set_settingsmerge_muted: (a: number, b: number) => void;
  readonly settingsmerge_new: () => number;
  readonly __wbg_settings_free: (a: number) => void;
  readonly __wbg_get_settings_scroll_speed: (a: number) => number;
  readonly __wbg_set_settings_scroll_speed: (a: number, b: number) => void;
  readonly __wbg_get_settings_judge_position: (a: number) => number;
  readonly __wbg_set_settings_judge_position: (a: number, b: number) => void;
  readonly __wbg_get_settings_scroll_direction: (a: number) => number;
  readonly __wbg_set_settings_scroll_direction: (a: number, b: number) => void;
  readonly __wbg_get_settings_lane_gap: (a: number) => number;
  readonly __wbg_set_settings_lane_gap: (a: number, b: number) => void;
  readonly __wbg_get_settings_muted: (a: number) => number;
  readonly __wbg_set_settings_muted: (a: number, b: number) => void;
  readonly __wbg_settingsmerge_free: (a: number) => void;
  readonly __wbg_engine_free: (a: number) => void;
  readonly engine_new: () => number;
  readonly engine_modify_settings: (a: number, b: number) => number;
  readonly engine_toJSON: (a: number) => number;
  readonly wasm_startup: () => void;
  readonly play: () => void;
  readonly main: (a: number, b: number) => number;
  readonly wgpu_render_pass_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_compute_pass_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_compute_pass_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_compute_pass_set_push_constant: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_compute_pass_dispatch_workgroups: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_compute_pass_dispatch_workgroups_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_push_debug_group: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_pop_debug_group: (a: number) => void;
  readonly wgpu_compute_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_write_timestamp: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_begin_pipeline_statistics_query: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_end_pipeline_statistics_query: (a: number) => void;
  readonly wgpu_render_bundle_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_render_bundle_set_vertex_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_index_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_draw: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_bundle_draw_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_draw_indexed_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_pop_debug_group: (a: number) => void;
  readonly wgpu_render_bundle_insert_debug_marker: (a: number, b: number) => void;
  readonly wgpu_render_bundle_push_debug_group: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_vertex_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_index_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_blend_constant: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_stencil_reference: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_viewport: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly wgpu_render_pass_set_scissor_rect: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_draw: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_draw_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_draw_indexed_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_multi_draw_indirect: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_render_pass_multi_draw_indexed_indirect: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_render_pass_multi_draw_indirect_count: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_multi_draw_indexed_indirect_count: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_push_debug_group: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_pop_debug_group: (a: number) => void;
  readonly wgpu_render_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_write_timestamp: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_begin_pipeline_statistics_query: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_end_pipeline_statistics_query: (a: number) => void;
  readonly wgpu_render_pass_execute_bundles: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb16c9e3a94cd67df: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0338c886a80d1831: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1b2711bae47534a7: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6eda11db1a9d597a: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he53239e32595ff2a: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h34d8a560434514f5: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;