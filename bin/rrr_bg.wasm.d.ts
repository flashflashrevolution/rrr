/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_get_settingsmerge_scroll_speed(a: number): number;
export function __wbg_set_settingsmerge_scroll_speed(a: number, b: number): void;
export function __wbg_get_settingsmerge_judge_position(a: number, b: number): void;
export function __wbg_set_settingsmerge_judge_position(a: number, b: number, c: number): void;
export function __wbg_get_settingsmerge_scroll_direction(a: number): number;
export function __wbg_set_settingsmerge_scroll_direction(a: number, b: number): void;
export function __wbg_get_settingsmerge_lane_gap(a: number): number;
export function __wbg_set_settingsmerge_lane_gap(a: number, b: number): void;
export function __wbg_get_settingsmerge_muted(a: number): number;
export function __wbg_set_settingsmerge_muted(a: number, b: number): void;
export function settingsmerge_new(): number;
export function __wbg_settings_free(a: number): void;
export function __wbg_get_settings_scroll_speed(a: number): number;
export function __wbg_set_settings_scroll_speed(a: number, b: number): void;
export function __wbg_get_settings_judge_position(a: number): number;
export function __wbg_set_settings_judge_position(a: number, b: number): void;
export function __wbg_get_settings_scroll_direction(a: number): number;
export function __wbg_set_settings_scroll_direction(a: number, b: number): void;
export function __wbg_get_settings_lane_gap(a: number): number;
export function __wbg_set_settings_lane_gap(a: number, b: number): void;
export function __wbg_get_settings_muted(a: number): number;
export function __wbg_set_settings_muted(a: number, b: number): void;
export function __wbg_settingsmerge_free(a: number): void;
export function __wbg_engine_free(a: number): void;
export function engine_new(): number;
export function engine_modify_settings(a: number, b: number): number;
export function engine_toJSON(a: number): number;
export function wasm_startup(): void;
export function play(): void;
export function main(a: number, b: number): number;
export function wgpu_render_pass_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_compute_pass_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_compute_pass_set_pipeline(a: number, b: number): void;
export function wgpu_compute_pass_set_push_constant(a: number, b: number, c: number, d: number): void;
export function wgpu_compute_pass_dispatch_workgroups(a: number, b: number, c: number, d: number): void;
export function wgpu_compute_pass_dispatch_workgroups_indirect(a: number, b: number, c: number): void;
export function wgpu_compute_pass_push_debug_group(a: number, b: number, c: number): void;
export function wgpu_compute_pass_pop_debug_group(a: number): void;
export function wgpu_compute_pass_insert_debug_marker(a: number, b: number, c: number): void;
export function wgpu_compute_pass_write_timestamp(a: number, b: number, c: number): void;
export function wgpu_compute_pass_begin_pipeline_statistics_query(a: number, b: number, c: number): void;
export function wgpu_compute_pass_end_pipeline_statistics_query(a: number): void;
export function wgpu_render_bundle_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_set_pipeline(a: number, b: number): void;
export function wgpu_render_bundle_set_vertex_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_set_index_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_set_push_constants(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_draw(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_draw_indexed(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_bundle_draw_indirect(a: number, b: number, c: number): void;
export function wgpu_render_bundle_draw_indexed_indirect(a: number, b: number, c: number): void;
export function wgpu_render_bundle_pop_debug_group(a: number): void;
export function wgpu_render_bundle_insert_debug_marker(a: number, b: number): void;
export function wgpu_render_bundle_push_debug_group(a: number, b: number): void;
export function wgpu_render_pass_set_pipeline(a: number, b: number): void;
export function wgpu_render_pass_set_vertex_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_index_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_blend_constant(a: number, b: number): void;
export function wgpu_render_pass_set_stencil_reference(a: number, b: number): void;
export function wgpu_render_pass_set_viewport(a: number, b: number, c: number, d: number, e: number, f: number, g: number): void;
export function wgpu_render_pass_set_scissor_rect(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_push_constants(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_draw(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_draw_indexed(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_draw_indirect(a: number, b: number, c: number): void;
export function wgpu_render_pass_draw_indexed_indirect(a: number, b: number, c: number): void;
export function wgpu_render_pass_multi_draw_indirect(a: number, b: number, c: number, d: number): void;
export function wgpu_render_pass_multi_draw_indexed_indirect(a: number, b: number, c: number, d: number): void;
export function wgpu_render_pass_multi_draw_indirect_count(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_multi_draw_indexed_indirect_count(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_push_debug_group(a: number, b: number, c: number): void;
export function wgpu_render_pass_pop_debug_group(a: number): void;
export function wgpu_render_pass_insert_debug_marker(a: number, b: number, c: number): void;
export function wgpu_render_pass_write_timestamp(a: number, b: number, c: number): void;
export function wgpu_render_pass_begin_pipeline_statistics_query(a: number, b: number, c: number): void;
export function wgpu_render_pass_end_pipeline_statistics_query(a: number): void;
export function wgpu_render_pass_execute_bundles(a: number, b: number, c: number): void;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export const __wbindgen_export_2: WebAssembly.Table;
export function _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb16c9e3a94cd67df(a: number, b: number): void;
export function _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0338c886a80d1831(a: number, b: number, c: number): void;
export function _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1b2711bae47534a7(a: number, b: number): void;
export function _dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6eda11db1a9d597a(a: number, b: number, c: number): void;
export function _dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he53239e32595ff2a(a: number, b: number): void;
export function _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h34d8a560434514f5(a: number, b: number, c: number): void;
export function __wbindgen_free(a: number, b: number): void;
export function __wbindgen_exn_store(a: number): void;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_start(): void;
