(window.webpackJsonp=window.webpackJsonp||[]).push([[0],[,function(t,e,n){"use strict";n.r(e),n.d(e,"__wbg_beginPath_a32288087b69409d",function(){return d}),n.d(e,"__wbg_setlinewidth_28fe82dd4bca8684",function(){return w}),n.d(e,"__wbg_setstrokestyle_00e1f80f058318e0",function(){return b}),n.d(e,"__wbg_moveTo_ca0bf13ec3d67dd2",function(){return v}),n.d(e,"__wbg_lineTo_defb6d9db054319b",function(){return C}),n.d(e,"__wbg_arc_558f036118ae1672",function(){return x}),n.d(e,"__wbg_stroke_c9d45b70b0e6bfe3",function(){return E}),n.d(e,"Universe",function(){return R}),n.d(e,"__wbindgen_object_drop_ref",function(){return k}),n.d(e,"__wbindgen_throw",function(){return j});var r=n(2);const o=[{obj:void 0},{obj:null},{obj:!0},{obj:!1}];let i=o.length;const s=CanvasRenderingContext2D.prototype.beginPath||function(){throw new Error("wasm-bindgen: CanvasRenderingContext2D.prototype.beginPath does not exist")},u=[];function c(t){if(1==(1&t))return u[t>>1];return o[t>>1].obj}function d(t){s.call(c(t))}function a(t,e){for(;t;){let n=Object.getOwnPropertyDescriptor(t,e);if(n)return n;t=Object.getPrototypeOf(t)}throw new Error(`descriptor for id='${e}' not found`)}const _=a(CanvasRenderingContext2D.prototype,"lineWidth").set||function(){throw new Error("wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(CanvasRenderingContext2D.prototype, 'lineWidth').set does not exist")};function w(t,e){_.call(c(t),e)}const p=a(CanvasRenderingContext2D.prototype,"strokeStyle").set||function(){throw new Error("wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(CanvasRenderingContext2D.prototype, 'strokeStyle').set does not exist")};let f=new TextDecoder("utf-8"),h=null;function g(t,e){return f.decode((null!==h&&h.buffer===r.memory.buffer||(h=new Uint8Array(r.memory.buffer)),h).subarray(t,t+e))}function b(t,e,n){let r=g(e,n);p.call(c(t),r)}const l=CanvasRenderingContext2D.prototype.moveTo||function(){throw new Error("wasm-bindgen: CanvasRenderingContext2D.prototype.moveTo does not exist")};function v(t,e,n){l.call(c(t),e,n)}const m=CanvasRenderingContext2D.prototype.lineTo||function(){throw new Error("wasm-bindgen: CanvasRenderingContext2D.prototype.lineTo does not exist")};function C(t,e,n){m.call(c(t),e,n)}const y=CanvasRenderingContext2D.prototype.arc||function(){throw new Error("wasm-bindgen: CanvasRenderingContext2D.prototype.arc does not exist")};function x(t,e,n,r,o,i,s){y.call(c(t),e,n,r,o,i,0!==s)}const D=CanvasRenderingContext2D.prototype.stroke||function(){throw new Error("wasm-bindgen: CanvasRenderingContext2D.prototype.stroke does not exist")};function E(t){D.call(c(t))}class R{static __construct(t){return new R(t)}constructor(t){this.ptr=t}get width(){if(0===this.ptr)throw new Error("Attempt to use a moved value");return r.__wbg_get_universe_width(this.ptr)}set width(t){if(0===this.ptr)throw new Error("Attempt to use a moved value");return r.__wbg_set_universe_width(this.ptr,t)}get height(){if(0===this.ptr)throw new Error("Attempt to use a moved value");return r.__wbg_get_universe_height(this.ptr)}set height(t){if(0===this.ptr)throw new Error("Attempt to use a moved value");return r.__wbg_set_universe_height(this.ptr,t)}free(){const t=this.ptr;this.ptr=0,function(t){r.__wbg_universe_free(t)}(t)}static new(t){return R.__construct(r.universe_new(function(t){i===o.length&&o.push(o.length+1);const e=i,n=o[e];return i=n,o[e]={obj:t,cnt:1},e<<1}(t)))}clicked(t,e,n,o){if(0===this.ptr)throw new Error("Attempt to use a moved value");return 0!==r.universe_clicked(this.ptr,t,e,n,o)}draw_grid(){if(0===this.ptr)throw new Error("Attempt to use a moved value");return r.universe_draw_grid(this.ptr)}draw_points(){if(0===this.ptr)throw new Error("Attempt to use a moved value");return r.universe_draw_points(this.ptr)}}function k(t){!function(t){if((t>>=1)<4)return;let e=o[t];e.cnt-=1,e.cnt>0||(o[t]=i,i=t)}(t)}function j(t,e){throw new Error(g(t,e))}},function(t,e,n){"use strict";var r=n.w[t.i];for(var o in n.r(e),r)"__webpack_init__"!=o&&(e[o]=r[o]);n(1);r.__webpack_init__()}]]);