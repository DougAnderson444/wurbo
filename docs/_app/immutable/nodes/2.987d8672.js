import{_ as B}from"../chunks/preload-helper.41c905a7.js";import{s as E,n as h,o as L,t as S,b as C}from"../chunks/scheduler.e108d1fd.js";import{S as k,i as D,g as H,h as I,j as M,f as l,a as g,H as P,e as w,y as R}from"../chunks/index.ee15a0a9.js";const q=""+new URL("../assets/vowels.d916278d.wasm",import.meta.url).href,T=`export function prnt(string) {\r
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
		// Because this cod eis in a blob:URL, we can't access the window object\r
		// directly. Instead, we post a message to the parent window\r
		bc.postMessage(window.render(e.target.value));\r
	});\r
}\r
`;function _(a){let n,e;return{c(){n=new P(!1),e=w(),this.h()},l(t){n=R(t,!1),e=w(),this.h()},h(){n.a=e},m(t,r){n.m(a[0],t,r),g(t,e,r)},p(t,r){r&1&&n.p(t[0])},d(t){t&&(l(e),n.d())}}}function U(a){let n,e=a[0]&&_(a);return{c(){n=H("div"),e&&e.c()},l(t){n=I(t,"DIV",{});var r=M(n);e&&e.l(r),r.forEach(l)},m(t,r){g(t,n,r),e&&e.m(n,null),a[3](n)},p(t,[r]){t[0]?e?e.p(t,r):(e=_(t),e.c(),e.m(n,null)):e&&(e.d(1),e=null)},i:h,o:h,d(t){t&&l(n),e&&e.d(),a[3](null)}}}function j(a,n,e){let t,r,o;L(async()=>{const i=new BroadcastChannel("listener_updates"),{load:b}=await B(()=>import("../chunks/index.ca7d20d0.js"),["../chunks/index.ca7d20d0.js","../chunks/preload-helper.41c905a7.js"],import.meta.url);let v=await fetch(q).then(s=>s.arrayBuffer()),d=[{"wurbo:vowels/imports":T}];console.log({importables:d}),e(2,o=await b(v,d)),e(0,t=o.render("World")),window.render=o.render,i.onmessage=s=>{var m,u,f;let c=((f=(u=(m=new DOMParser().parseFromString(s.data||"","text/html"))==null?void 0:m.body)==null?void 0:u.firstElementChild)==null?void 0:f.id)||null;if(c){let p=document.getElementById(c);if(p){p.outerHTML=s.data;return}}e(0,t=s.data)}});function y(i){C[i?"unshift":"push"](()=>{r=i,e(1,r)})}return a.$$.update=()=>{a.$$.dirty&5&&t&&o&&(async()=>(await S(),o.listen()))()},[t,r,o,y]}class F extends k{constructor(n){super(),D(this,n,j,U,E,{})}}export{F as component};
