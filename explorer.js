(()=>{var n={511:(n,e,t)=>{t(154),t(190),n.exports=""},190:(n,e,t)=>{"use strict";t.r(e)},154:(n,e,t)=>{"use strict";let r;t.r(e);const _=new Array(32).fill(void 0);function o(n){return _[n]}_.push(void 0,null,!0,!1);let c=_.length;function i(n){const e=o(n);return function(n){n<36||(_[n]=c,c=n)}(n),e}function u(n){c===_.length&&_.push(_.length+1);const e=c;return c=_[e],_[e]=n,e}const b=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});b.decode();let a=new Uint8Array;function f(){return 0===a.byteLength&&(a=new Uint8Array(r.memory.buffer)),a}function w(n,e){return b.decode(f().subarray(n,n+e))}function s(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=s(n[0]));for(let r=1;r<e;r++)t+=", "+s(n[r]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let r;if(!(t.length>1))return toString.call(n);if(r=t[1],"Object"==r)try{return"Object("+JSON.stringify(n)+")"}catch(n){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:r}let g=0;const l=new TextEncoder("utf-8"),d="function"==typeof l.encodeInto?function(n,e){return l.encodeInto(n,e)}:function(n,e){const t=l.encode(n);return e.set(t),{read:n.length,written:t.length}};function y(n,e,t){if(void 0===t){const t=l.encode(n),r=e(t.length);return f().subarray(r,r+t.length).set(t),g=t.length,r}let r=n.length,_=e(r);const o=f();let c=0;for(;c<r;c++){const e=n.charCodeAt(c);if(e>127)break;o[_+c]=e}if(c!==r){0!==c&&(n=n.slice(c)),_=t(_,r,r=c+3*n.length);const e=f().subarray(_+c,_+r);c+=d(n,e).written}return g=c,_}let m=new Int32Array;function p(){return 0===m.byteLength&&(m=new Int32Array(r.memory.buffer)),m}function h(n){return null==n}let v=new Float64Array;function A(n,e,t,_){const o={a:n,b:e,cnt:1,dtor:t},c=(...n)=>{o.cnt++;const e=o.a;o.a=0;try{return _(e,o.b,...n)}finally{0==--o.cnt?r.__wbindgen_export_2.get(o.dtor)(e,o.b):o.a=e}};return c.original=o,c}function T(n,e){r._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4666c3823359531a(n,e)}function x(n,e,t){r._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hdda3a85e16c1c2ab(n,e,u(t))}function E(n,e,t){r._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h44a52bc6f95b52a7(n,e,u(t))}let R=new Uint32Array;function S(n,e){try{return n.apply(this,e)}catch(n){r.__wbindgen_exn_store(u(n))}}function O(){const n={};return n.wbg={},n.wbg.__wbg_document_3ead31dbcad65886=function(n){const e=o(n).document;return h(e)?0:u(e)},n.wbg.__wbindgen_object_drop_ref=function(n){i(n)},n.wbg.__wbg_createTextNode_300f845fab76642f=function(n,e,t){return u(o(n).createTextNode(w(e,t)))},n.wbg.__wbindgen_object_clone_ref=function(n){return u(o(n))},n.wbg.__wbg_new_abda76e883ba8a5f=function(){return u(new Error)},n.wbg.__wbg_stack_658279fe44541cf6=function(n,e){const t=y(o(e).stack,r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;p()[n/4+1]=_,p()[n/4+0]=t},n.wbg.__wbg_error_f851667af71bcfc6=function(n,e){try{console.error(w(n,e))}finally{r.__wbindgen_free(n,e)}},n.wbg.__wbg_body_3cb4b4042b9a632b=function(n){const e=o(n).body;return h(e)?0:u(e)},n.wbg.__wbg_lastChild_a2f5ed739809bb31=function(n){const e=o(n).lastChild;return h(e)?0:u(e)},n.wbg.__wbg_removeChild_6751e9ca5d9aaf00=function(){return S((function(n,e){return u(o(n).removeChild(o(e)))}),arguments)},n.wbg.__wbg_clearTimeout_5b4145302d77e5f3="function"==typeof clearTimeout?clearTimeout:("clearTimeout",()=>{throw new Error("clearTimeout is not defined")}),n.wbg.__wbindgen_cb_drop=function(n){const e=i(n).original;return 1==e.cnt--&&(e.a=0,!0)},n.wbg.__wbg_setTimeout_02c3975efb677088=function(){return S((function(n,e){return setTimeout(o(n),e)}),arguments)},n.wbg.__wbg_self_6d479506f72c6a71=function(){return S((function(){return u(self.self)}),arguments)},n.wbg.__wbg_window_f2557cc78490aceb=function(){return S((function(){return u(window.window)}),arguments)},n.wbg.__wbg_globalThis_7f206bda628d5286=function(){return S((function(){return u(globalThis.globalThis)}),arguments)},n.wbg.__wbg_global_ba75c50d1cf384f4=function(){return S((function(){return u(t.g.global)}),arguments)},n.wbg.__wbindgen_is_undefined=function(n){return void 0===o(n)},n.wbg.__wbg_newnoargs_b5b063fc6c2f0376=function(n,e){return u(new Function(w(n,e)))},n.wbg.__wbg_call_97ae9d8645dc388b=function(){return S((function(n,e){return u(o(n).call(o(e)))}),arguments)},n.wbg.__wbindgen_string_new=function(n,e){return u(w(n,e))},n.wbg.__wbg_new_0b9bfdd97583284e=function(){return u(new Object)},n.wbg.__wbg_set_bf3f89b92d5a34bf=function(){return S((function(n,e,t){return Reflect.set(o(n),o(e),o(t))}),arguments)},n.wbg.__wbg_newwithstrandinit_05d7180788420c40=function(){return S((function(n,e,t){return u(new Request(w(n,e),o(t)))}),arguments)},n.wbg.__wbg_fetch_0fe04905cccfc2aa=function(n,e){return u(o(n).fetch(o(e)))},n.wbg.__wbg_instanceof_Response_eaa426220848a39e=function(n){let e;try{e=o(n)instanceof Response}catch{e=!1}return e},n.wbg.__wbg_ok_b8130e51d893123a=function(n){return o(n).ok},n.wbg.__wbg_statusText_7f6b7d97e47933bd=function(n,e){const t=y(o(e).statusText,r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;p()[n/4+1]=_,p()[n/4+0]=t},n.wbg.__wbg_arrayBuffer_4c27b6f00c530232=function(){return S((function(n){return u(o(n).arrayBuffer())}),arguments)},n.wbg.__wbg_new_8c3f0052272a457a=function(n){return u(new Uint8Array(o(n)))},n.wbg.__wbg_length_9e1ae1900cb0fbd5=function(n){return o(n).length},n.wbg.__wbindgen_memory=function(){return u(r.memory)},n.wbg.__wbg_buffer_3f3d764d4747d564=function(n){return u(o(n).buffer)},n.wbg.__wbg_set_83db9690f9353e79=function(n,e,t){o(n).set(o(e),t>>>0)},n.wbg.__wbindgen_debug_string=function(n,e){const t=y(s(o(e)),r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;p()[n/4+1]=_,p()[n/4+0]=t},n.wbg.__wbindgen_throw=function(n,e){throw new Error(w(n,e))},n.wbg.__wbg_then_11f7a54d67b4bfad=function(n,e){return u(o(n).then(o(e)))},n.wbg.__wbg_then_cedad20fbbd9418a=function(n,e,t){return u(o(n).then(o(e),o(t)))},n.wbg.__wbg_resolve_99fe17964f31ffc0=function(n){return u(Promise.resolve(o(n)))},n.wbg.__wbg_instanceof_Window_acc97ff9f5d2c7b4=function(n){let e;try{e=o(n)instanceof Window}catch{e=!1}return e},n.wbg.__wbg_value_ccb32485ee1b3928=function(n,e){const t=y(o(e).value,r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;p()[n/4+1]=_,p()[n/4+0]=t},n.wbg.__wbg_target_bf704b7db7ad1387=function(n){const e=o(n).target;return h(e)?0:u(e)},n.wbg.__wbg_instanceof_Element_33bd126d58f2021b=function(n){let e;try{e=o(n)instanceof Element}catch{e=!1}return e},n.wbg.__wbg_cancelBubble_8c0bdf21c08f1717=function(n){return o(n).cancelBubble},n.wbg.__wbg_parentElement_0cffb3ceb0f107bd=function(n){const e=o(n).parentElement;return h(e)?0:u(e)},n.wbg.__wbg_get_765201544a2b6869=function(){return S((function(n,e){return u(Reflect.get(o(n),o(e)))}),arguments)},n.wbg.__wbindgen_number_get=function(n,e){const t=o(e),_="number"==typeof t?t:void 0;(0===v.byteLength&&(v=new Float64Array(r.memory.buffer)),v)[n/8+1]=h(_)?0:_,p()[n/4+0]=!h(_)},n.wbg.__wbg_valueOf_6b6effad03e5c546=function(n){return o(n).valueOf()},n.wbg.__wbg_removeAttribute_beaed7727852af78=function(){return S((function(n,e,t){o(n).removeAttribute(w(e,t))}),arguments)},n.wbg.__wbg_setAttribute_d8436c14a59ab1af=function(){return S((function(n,e,t,r,_){o(n).setAttribute(w(e,t),w(r,_))}),arguments)},n.wbg.__wbg_appendChild_e513ef0e5098dfdd=function(){return S((function(n,e){return u(o(n).appendChild(o(e)))}),arguments)},n.wbg.__wbg_insertBefore_9f2d2defb9471006=function(){return S((function(n,e,t){return u(o(n).insertBefore(o(e),o(t)))}),arguments)},n.wbg.__wbg_setchecked_f1e1f3e62cdca8e7=function(n,e){o(n).checked=0!==e},n.wbg.__wbg_setvalue_e5b519cca37d82a7=function(n,e,t){o(n).value=w(e,t)},n.wbg.__wbg_value_b2a620d34c663701=function(n,e){const t=y(o(e).value,r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;p()[n/4+1]=_,p()[n/4+0]=t},n.wbg.__wbg_namespaceURI_e19c7be2c60e5b5c=function(n,e){const t=o(e).namespaceURI;var _=h(t)?0:y(t,r.__wbindgen_malloc,r.__wbindgen_realloc),c=g;p()[n/4+1]=c,p()[n/4+0]=_},n.wbg.__wbg_createElement_976dbb84fe1661b5=function(){return S((function(n,e,t){return u(o(n).createElement(w(e,t)))}),arguments)},n.wbg.__wbg_createElementNS_1561aca8ee3693c0=function(){return S((function(n,e,t,r,_){return u(o(n).createElementNS(0===e?void 0:w(e,t),w(r,_)))}),arguments)},n.wbg.__wbg_addEventListener_1fc744729ac6dc27=function(){return S((function(n,e,t,r,_){o(n).addEventListener(w(e,t),o(r),o(_))}),arguments)},n.wbg.__wbindgen_number_new=function(n){return u(n)},n.wbg.__wbg_warn_0b90a269a514ae1d=function(n,e){var t=function(n,e){const t=(0===R.byteLength&&(R=new Uint32Array(r.memory.buffer)),R).subarray(n/4,n/4+e),_=[];for(let n=0;n<t.length;n++)_.push(i(t[n]));return _}(n,e).slice();r.__wbindgen_free(n,4*e),console.warn(...t)},n.wbg.__wbg_setvalue_df64bc6794c098f2=function(n,e,t){o(n).value=w(e,t)},n.wbg.__wbg_setnodeValue_4077cafeefd0725e=function(n,e,t){o(n).nodeValue=0===e?void 0:w(e,t)},n.wbg.__wbg_is_40a66842732708e7=function(n,e){return Object.is(o(n),o(e))},n.wbg.__wbindgen_closure_wrapper320=function(n,e,t){return u(A(n,e,18,T))},n.wbg.__wbindgen_closure_wrapper924=function(n,e,t){return u(A(n,e,18,x))},n.wbg.__wbindgen_closure_wrapper1475=function(n,e,t){const _=function(n,e,t,_){const o={a:n,b:e,cnt:1,dtor:18},c=(...n)=>{o.cnt++;try{return _(o.a,o.b,...n)}finally{0==--o.cnt&&(r.__wbindgen_export_2.get(o.dtor)(o.a,o.b),o.a=0)}};return c.original=o,c}(n,e,0,E);return u(_)},n}async function j(n){void 0===n&&(n=new URL(t(198),t.b));const e=O();("string"==typeof n||"function"==typeof Request&&n instanceof Request||"function"==typeof URL&&n instanceof URL)&&(n=fetch(n));const{instance:_,module:o}=await async function(n,e){if("function"==typeof Response&&n instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(n,e)}catch(e){if("application/wasm"==n.headers.get("Content-Type"))throw e;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e)}const t=await n.arrayBuffer();return await WebAssembly.instantiate(t,e)}{const t=await WebAssembly.instantiate(n,e);return t instanceof WebAssembly.Instance?{instance:t,module:n}:t}}(await n,e);return function(n,e){return r=n.exports,j.__wbindgen_wasm_module=e,v=new Float64Array,m=new Int32Array,R=new Uint32Array,a=new Uint8Array,r.__wbindgen_start(),r}(_,o)}const U=j;(async function(){await U(),function(n){const e=y(await async function(){return"https://ffxiv-data.dlunch.net/compressed"}(),r.__wbindgen_malloc,r.__wbindgen_realloc),t=g;r.start(e,t)}()})().catch(console.error)},198:(n,e,t)=>{"use strict";n.exports=t.p+"2719891c0696acfa.wasm"}},e={};function t(r){var _=e[r];if(void 0!==_)return _.exports;var o=e[r]={exports:{}};return n[r](o,o.exports,t),o.exports}t.m=n,t.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(n){if("object"==typeof window)return window}}(),t.o=(n,e)=>Object.prototype.hasOwnProperty.call(n,e),t.r=n=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(n,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(n,"__esModule",{value:!0})},(()=>{var n;t.g.importScripts&&(n=t.g.location+"");var e=t.g.document;if(!n&&e&&(e.currentScript&&(n=e.currentScript.src),!n)){var r=e.getElementsByTagName("script");r.length&&(n=r[r.length-1].src)}if(!n)throw new Error("Automatic publicPath is not supported in this browser");n=n.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),t.p=n})(),t.b=document.baseURI||self.location.href,t(511)})();