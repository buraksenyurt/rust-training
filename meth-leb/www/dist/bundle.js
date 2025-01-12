/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _pkg_meth_leb_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../pkg/meth_leb.js */ \"../pkg/meth_leb.js\");\n\r\n\r\nasync function main() {\r\n    await (0,_pkg_meth_leb_js__WEBPACK_IMPORTED_MODULE_0__[\"default\"])();\r\n\r\n    const form = document.getElementById('form');\r\n    const canvas = document.getElementById('canvas');\r\n    const context = canvas.getContext('2d');\r\n\r\n    form.addEventListener('submit', (event) => {\r\n        event.preventDefault();\r\n\r\n        const a = parseFloat(document.getElementById('a').value);\r\n        const b = parseFloat(document.getElementById('b').value);\r\n        const c = parseFloat(document.getElementById('c').value);\r\n        const x_min = parseFloat(document.getElementById('x_min').value);\r\n        const x_max = parseFloat(document.getElementById('x_max').value);\r\n        const steps = parseInt(document.getElementById('steps').value);\r\n\r\n        const points = (0,_pkg_meth_leb_js__WEBPACK_IMPORTED_MODULE_0__.calculate_parabola)(a, b, c, x_min, x_max, steps);\r\n\r\n        context.clearRect(0, 0, canvas.width, canvas.height);\r\n        context.beginPath();\r\n        context.moveTo((points[0] - x_min) * 50, 300 - points[1] * 50);\r\n\r\n        for (let i = 2; i < points.length; i += 2) {\r\n            const x = (points[i] - x_min) * 50;\r\n            const y = 300 - points[i + 1] * 50;\r\n            context.lineTo(x, y);\r\n        }\r\n\r\n        context.stroke();\r\n    });\r\n}\r\n\r\nmain()\r\n    .then(() => console.log(\"Parabola processed successfully!\"))\r\n    .catch((error) => console.error(\"Error on main function\", error));\r\n\n\n//# sourceURL=webpack://parabol-example/./index.js?");

/***/ }),

/***/ "../pkg/meth_leb_bg.wasm":
/*!*******************************!*\
  !*** ../pkg/meth_leb_bg.wasm ***!
  \*******************************/
/***/ ((module, __unused_webpack_exports, __webpack_require__) => {

eval("module.exports = __webpack_require__.p + \"85e05c94123b4ab615ea.wasm\";\n\n//# sourceURL=webpack://parabol-example/../pkg/meth_leb_bg.wasm?");

/***/ }),

/***/ "../pkg/meth_leb.js":
/*!**************************!*\
  !*** ../pkg/meth_leb.js ***!
  \**************************/
/***/ ((__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   calculate_parabola: () => (/* binding */ calculate_parabola),\n/* harmony export */   \"default\": () => (__WEBPACK_DEFAULT_EXPORT__),\n/* harmony export */   greet: () => (/* binding */ greet),\n/* harmony export */   initSync: () => (/* binding */ initSync)\n/* harmony export */ });\nlet wasm;\n\nconst cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );\n\nif (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };\n\nlet cachedUint8ArrayMemory0 = null;\n\nfunction getUint8ArrayMemory0() {\n    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {\n        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8ArrayMemory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));\n}\n\nfunction greet() {\n    wasm.greet();\n}\n\nlet cachedFloat64ArrayMemory0 = null;\n\nfunction getFloat64ArrayMemory0() {\n    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {\n        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);\n    }\n    return cachedFloat64ArrayMemory0;\n}\n\nfunction getArrayF64FromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);\n}\n/**\n * @param {number} a\n * @param {number} b\n * @param {number} c\n * @param {number} x_min\n * @param {number} x_max\n * @param {number} steps\n * @returns {Float64Array}\n */\nfunction calculate_parabola(a, b, c, x_min, x_max, steps) {\n    const ret = wasm.calculate_parabola(a, b, c, x_min, x_max, steps);\n    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();\n    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);\n    return v1;\n}\n\nasync function __wbg_load(module, imports) {\n    if (typeof Response === 'function' && module instanceof Response) {\n        if (typeof WebAssembly.instantiateStreaming === 'function') {\n            try {\n                return await WebAssembly.instantiateStreaming(module, imports);\n\n            } catch (e) {\n                if (module.headers.get('Content-Type') != 'application/wasm') {\n                    console.warn(\"`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\\n\", e);\n\n                } else {\n                    throw e;\n                }\n            }\n        }\n\n        const bytes = await module.arrayBuffer();\n        return await WebAssembly.instantiate(bytes, imports);\n\n    } else {\n        const instance = await WebAssembly.instantiate(module, imports);\n\n        if (instance instanceof WebAssembly.Instance) {\n            return { instance, module };\n\n        } else {\n            return instance;\n        }\n    }\n}\n\nfunction __wbg_get_imports() {\n    const imports = {};\n    imports.wbg = {};\n    imports.wbg.__wbg_alert_793899edbcefb021 = function(arg0, arg1) {\n        alert(getStringFromWasm0(arg0, arg1));\n    };\n    imports.wbg.__wbindgen_init_externref_table = function() {\n        const table = wasm.__wbindgen_export_0;\n        const offset = table.grow(4);\n        table.set(0, undefined);\n        table.set(offset + 0, undefined);\n        table.set(offset + 1, null);\n        table.set(offset + 2, true);\n        table.set(offset + 3, false);\n        ;\n    };\n\n    return imports;\n}\n\nfunction __wbg_init_memory(imports, memory) {\n\n}\n\nfunction __wbg_finalize_init(instance, module) {\n    wasm = instance.exports;\n    __wbg_init.__wbindgen_wasm_module = module;\n    cachedFloat64ArrayMemory0 = null;\n    cachedUint8ArrayMemory0 = null;\n\n\n    wasm.__wbindgen_start();\n    return wasm;\n}\n\nfunction initSync(module) {\n    if (wasm !== undefined) return wasm;\n\n\n    if (typeof module !== 'undefined') {\n        if (Object.getPrototypeOf(module) === Object.prototype) {\n            ({module} = module)\n        } else {\n            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')\n        }\n    }\n\n    const imports = __wbg_get_imports();\n\n    __wbg_init_memory(imports);\n\n    if (!(module instanceof WebAssembly.Module)) {\n        module = new WebAssembly.Module(module);\n    }\n\n    const instance = new WebAssembly.Instance(module, imports);\n\n    return __wbg_finalize_init(instance, module);\n}\n\nasync function __wbg_init(module_or_path) {\n    if (wasm !== undefined) return wasm;\n\n\n    if (typeof module_or_path !== 'undefined') {\n        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {\n            ({module_or_path} = module_or_path)\n        } else {\n            console.warn('using deprecated parameters for the initialization function; pass a single object instead')\n        }\n    }\n\n    if (typeof module_or_path === 'undefined') {\n        module_or_path = new URL(/* asset import */ __webpack_require__(/*! meth_leb_bg.wasm */ \"../pkg/meth_leb_bg.wasm\"), __webpack_require__.b);\n    }\n    const imports = __wbg_get_imports();\n\n    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {\n        module_or_path = fetch(module_or_path);\n    }\n\n    __wbg_init_memory(imports);\n\n    const { instance, module } = await __wbg_load(await module_or_path, imports);\n\n    return __wbg_finalize_init(instance, module);\n}\n\n\n/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (__wbg_init);\n\n\n//# sourceURL=webpack://parabol-example/../pkg/meth_leb.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = __webpack_modules__;
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/global */
/******/ 	(() => {
/******/ 		__webpack_require__.g = (function() {
/******/ 			if (typeof globalThis === 'object') return globalThis;
/******/ 			try {
/******/ 				return this || new Function('return this')();
/******/ 			} catch (e) {
/******/ 				if (typeof window === 'object') return window;
/******/ 			}
/******/ 		})();
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/publicPath */
/******/ 	(() => {
/******/ 		var scriptUrl;
/******/ 		if (__webpack_require__.g.importScripts) scriptUrl = __webpack_require__.g.location + "";
/******/ 		var document = __webpack_require__.g.document;
/******/ 		if (!scriptUrl && document) {
/******/ 			if (document.currentScript && document.currentScript.tagName.toUpperCase() === 'SCRIPT')
/******/ 				scriptUrl = document.currentScript.src;
/******/ 			if (!scriptUrl) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				if(scripts.length) {
/******/ 					var i = scripts.length - 1;
/******/ 					while (i > -1 && (!scriptUrl || !/^http(s?):/.test(scriptUrl))) scriptUrl = scripts[i--].src;
/******/ 				}
/******/ 			}
/******/ 		}
/******/ 		// When supporting browsers where an automatic publicPath is not supported you must specify an output.publicPath manually via configuration
/******/ 		// or pass an empty string ("") and set the __webpack_public_path__ variable from your code to use your own logic.
/******/ 		if (!scriptUrl) throw new Error("Automatic publicPath is not supported in this browser");
/******/ 		scriptUrl = scriptUrl.replace(/#.*$/, "").replace(/\?.*$/, "").replace(/\/[^\/]+$/, "/");
/******/ 		__webpack_require__.p = scriptUrl;
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/jsonp chunk loading */
/******/ 	(() => {
/******/ 		__webpack_require__.b = document.baseURI || self.location.href;
/******/ 		
/******/ 		// object to store loaded and loading chunks
/******/ 		// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 		// [resolve, reject, Promise] = chunk loading, 0 = chunk loaded
/******/ 		var installedChunks = {
/******/ 			"main": 0
/******/ 		};
/******/ 		
/******/ 		// no chunk on demand loading
/******/ 		
/******/ 		// no prefetching
/******/ 		
/******/ 		// no preloaded
/******/ 		
/******/ 		// no HMR
/******/ 		
/******/ 		// no HMR manifest
/******/ 		
/******/ 		// no on chunks loaded
/******/ 		
/******/ 		// no jsonp function
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./index.js");
/******/ 	
/******/ })()
;