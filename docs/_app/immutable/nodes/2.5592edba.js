import{_ as w}from"../chunks/preload-helper.a4192956.js";import{s as f,n as u,o as g,t as v}from"../chunks/scheduler.e108d1fd.js";import{S as b,i as _,g as y,h as E,j as L,f as l,a as h,H as A,e as d,y as C}from"../chunks/index.649195e8.js";const N="listener_updates",B="wurbo";function M(r){window[B]={render:r.reactivity.render};const o=new BroadcastChannel(N);o.onmessage=t=>{var n,s,a;let e=((a=(s=(n=new DOMParser().parseFromString(t.data||"","text/html"))==null?void 0:n.body)==null?void 0:s.firstElementChild)==null?void 0:a.id)||null;if(e){let i=document.getElementById(e);if(i){i.outerHTML=t.data;return}}console.warn("No element found with id: ",e)}}const S=""+new URL("../assets/vowels.57844515.wasm",import.meta.url).href,H=`// This code must be stand alone and 100% resolved into this single file\r
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
		console.log('target value', e.target.value);\r
		console.log({ outputid });\r
		// The API has changed. So now we need to chg the call too...\r
		let output_ctx = {\r
			name: e.target.value,\r
			// the output id\r
			id: outputid\r
		};\r
		let ctx = {\r
			page: { title: "Let's count vowels using templates for Inputs and Outputs!" },\r
			input: { placeholder: "Input the word with vowels it's here" },\r
			output: output_ctx\r
		};\r
		console.log({ ctx });\r
		let msg = window.wurbo.render(ctx);\r
		console.log('sending message', msg);\r
		bc.postMessage(msg);\r
		// alternatively, bundle with the exported constant:\r
		// bc.postMessage(window[GLOBAL_NAMESPACE].render(e.target.value));\r
	});\r
}\r
`;function c(r){let o,t;return{c(){o=new A(!1),t=d(),this.h()},l(e){o=C(e,!1),t=d(),this.h()},h(){o.a=t},m(e,n){o.m(r[0],e,n),h(e,t,n)},p(e,n){n&1&&o.p(e[0])},d(e){e&&(l(t),o.d())}}}function I(r){let o,t=r[0]&&c(r);return{c(){o=y("div"),t&&t.c()},l(e){o=E(e,"DIV",{});var n=L(o);t&&t.l(n),n.forEach(l)},m(e,n){h(e,o,n),t&&t.m(o,null)},p(e,[n]){e[0]?t?t.p(e,n):(t=c(e),t.c(),t.m(o,null)):t&&(t.d(1),t=null)},i:u,o:u,d(e){e&&l(o),t&&t.d()}}}function T(r,o,t){let e,n;return g(async()=>{const{load:s}=await w(()=>import("../chunks/index.87c1ed15.js"),["../chunks/index.87c1ed15.js","../chunks/preload-helper.a4192956.js"],import.meta.url);let a=await fetch(S).then(p=>p.arrayBuffer());t(1,n=await s(a,[{"demo:vowels/imports":H}]));let m={page:{title:"Let's count vowels using templates for Inputs and Outputs!"},input:{placeholder:"Input the word with vowels it's here"},output:{name:"vowels"}};t(0,e=n.reactivity.render(m)),M(n)}),r.$$.update=()=>{r.$$.dirty&3&&e&&n&&(async()=>(await v(),n.reactivity.activate()))()},[e,n]}class k extends b{constructor(o){super(),_(this,o,T,I,f,{})}}export{k as component};
