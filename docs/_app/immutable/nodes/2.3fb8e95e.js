import{_ as w}from"../chunks/preload-helper.a4192956.js";import{s as f,n as d,o as g,t as v}from"../chunks/scheduler.e108d1fd.js";import{S as b,i as _,g as y,h as E,j as L,f as l,a as h,H as A,e as u,y as C}from"../chunks/index.649195e8.js";const N="listener_updates",B="wurbo";function M(o){window[B]={render:o.reactivity.render,update:o.reactivity.update};const r=new BroadcastChannel(N);r.onmessage=t=>{var n,a,s;let e=((s=(a=(n=new DOMParser().parseFromString(t.data||"","text/html"))==null?void 0:n.body)==null?void 0:a.firstElementChild)==null?void 0:s.id)||null;if(e){let i=document.getElementById(e);if(i){i.outerHTML=t.data;return}}console.warn("No element found with id: ",e)}}const H=""+new URL("../assets/vowels.5d8dca8d.wasm",import.meta.url).href,S=`// This code must be stand alone and 100% resolved into this single file\r
// as it gets stringified and rolled up into the bundle of BlobURL code that loads the wasm\r
\r
// You could alternatively use the constant, but this would involve a bundle step to resolve all the code into a single file\r
// import { CHANNEL_NAME, GLOBAL_NAMESPACE} from 'wurbo';\r
\r
export function prnt(string) {\r
	console.log(string);\r
}\r
\r
/**\r
 * @param {{selector: string, ty: string, value: string}} param0 - The CSS selector we want to listen on, ty is the event type, value\r
 * @param {function} handler - function to run when event is triggered\r
 */\r
export function addeventlistener({ selector, ty, outputid }) {\r
	// The Broadcast channel name must be the same as the Wurbo\r
	// We could import the constant from Wurbo, but this would involve a bundle step to resolve all the code into a single file\r
	// Here we use it by string to show how it works\r
	// If you want to import and bundle it, go right ahead\r
	// const bc = new BroadcastChannel(CHANNEL_NAME);\r
	const bc = new BroadcastChannel('listener_updates');\r
	document.querySelector(selector).addEventListener(ty, (e) => {\r
		// This code gets bundled into the main bundle, but there's no way for us\r
		// to reference \`render\` directly. But we can access the window object\r
		// which ensures we are using the same function. We post a message to the parent window with the result\r
		let output_ctx = {\r
			tag: 'output',\r
			val: {\r
				name: e.target.value,\r
				id: outputid\r
			}\r
		};\r
		bc.postMessage(window.wurbo.render(output_ctx));\r
		// alternatively, bundle with the exported constant:\r
		// bc.postMessage(window[GLOBAL_NAMESPACE].render(e.target.value));\r
	});\r
}\r
`;function c(o){let r,t;return{c(){r=new A(!1),t=u(),this.h()},l(e){r=C(e,!1),t=u(),this.h()},h(){r.a=t},m(e,n){r.m(o[0],e,n),h(e,t,n)},p(e,n){n&1&&r.p(e[0])},d(e){e&&(l(t),r.d())}}}function T(o){let r,t=o[0]&&c(o);return{c(){r=y("div"),t&&t.c()},l(e){r=E(e,"DIV",{});var n=L(r);t&&t.l(n),n.forEach(l)},m(e,n){h(e,r,n),t&&t.m(r,null)},p(e,[n]){e[0]?t?t.p(e,n):(t=c(e),t.c(),t.m(r,null)):t&&(t.d(1),t=null)},i:d,o:d,d(e){e&&l(r),t&&t.d()}}}function P(o,r,t){let e,n;return g(async()=>{const{load:a}=await w(()=>import("../chunks/index.87c1ed15.js"),["../chunks/index.87c1ed15.js","../chunks/preload-helper.a4192956.js"],import.meta.url);let s=await fetch(H).then(p=>p.arrayBuffer());t(1,n=await a(s,[{"demo:vowels/imports":S}]));let m={tag:"content",val:{page:{title:"Let's count vowels using templates for Inputs and Outputs!"},input:{placeholder:"Input the word with vowels it's here"},output:{name:"vowels"}}};t(0,e=n.reactivity.render(m)),M(n)}),o.$$.update=()=>{o.$$.dirty&3&&e&&n&&(async()=>(await v(),n.reactivity.activate()))()},[e,n]}class k extends b{constructor(r){super(),_(this,r,P,T,f,{})}}export{k as component};
