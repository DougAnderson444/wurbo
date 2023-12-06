import{_ as b}from"../chunks/preload-helper.41c905a7.js";import{s as y,n as u,o as v,t as B}from"../chunks/scheduler.e108d1fd.js";import{S as E,i as L,g as S,h as C,j as T,f as s,a as p,H as D,e as f,y as H}from"../chunks/index.ee15a0a9.js";const M=""+new URL("../assets/vowels.3c763261.wasm",import.meta.url).href,P=`// This  code must be stand alone (or bundled into a stand alone bundle)\r
// as it gets stringified and rolled up into the bundle of BobURL code that loads the wasm\r
export function prnt(string) {\r
	console.log(string);\r
}\r
/**\r
 * @param {string} ty - type of event to listen for\r
 * @param {function} handler - function to run when event is triggered\r
 */\r
export function addeventlistener({ selector, ty, value }) {\r
	const bc = new BroadcastChannel('listener_updates');\r
	let elem = document.querySelector(selector);\r
	document.querySelector(selector).addEventListener(ty, (e) => {\r
		// This code gets bundled into the main bundle, but there's no way for us\r
		// to reference \`render\` directly. But we can access the window object\r
		// which ensures we are using the same function. , we post a message to the parent window\r
		bc.postMessage(window.wurbo.render(e.target.value));\r
	});\r
}\r
`;function h(o){let r,e;return{c(){r=new D(!1),e=f(),this.h()},l(t){r=H(t,!1),e=f(),this.h()},h(){r.a=e},m(t,n){r.m(o[0],t,n),p(t,e,n)},p(t,n){n&1&&r.p(t[0])},d(t){t&&(s(e),r.d())}}}function R(o){let r,e=o[0]&&h(o);return{c(){r=S("div"),e&&e.c()},l(t){r=C(t,"DIV",{});var n=T(r);e&&e.l(n),n.forEach(s)},m(t,n){p(t,r,n),e&&e.m(r,null)},p(t,[n]){t[0]?e?e.p(t,n):(e=h(t),e.c(),e.m(r,null)):e&&(e.d(1),e=null)},i:u,o:u,d(t){t&&s(r),e&&e.d()}}}function k(o,r,e){let t,n;return v(async()=>{const w=new BroadcastChannel("listener_updates"),{load:g}=await b(()=>import("../chunks/index.ca7d20d0.js"),["../chunks/index.ca7d20d0.js","../chunks/preload-helper.41c905a7.js"],import.meta.url);let _=await fetch(M).then(a=>a.arrayBuffer());e(1,n=await g(_,[{"demo:vowels/imports":P}])),e(0,t=n.render("World")),window.wurbo={render:n.render},w.onmessage=a=>{var l,d,c;let i=((c=(d=(l=new DOMParser().parseFromString(a.data||"","text/html"))==null?void 0:l.body)==null?void 0:d.firstElementChild)==null?void 0:c.id)||null;if(i){let m=document.getElementById(i);if(m){m.outerHTML=a.data;return}}e(0,t=a.data)}}),o.$$.update=()=>{o.$$.dirty&3&&t&&n&&(async()=>(await B(),n.listen(),console.log(`listening timestamp ${Date.now()}`)))()},[t,n]}class x extends E{constructor(r){super(),L(this,r,k,R,y,{})}}export{x as component};
