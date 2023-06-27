import{_ as E}from"../chunks/preload-helper.41c905a7.js";import{s as S,n as h,o as C,t as L,b as I}from"../chunks/scheduler.e108d1fd.js";import{S as M,i as k,g as D,h as H,j as P,f as l,a as g,H as R,e as w,y as T}from"../chunks/index.ee15a0a9.js";const j=""+new URL("../assets/hello.ea1c9dfb.wasm",import.meta.url).href,q=`export const cargoCompImports = {\r
	prnt(string) {\r
		console.log(string);\r
	},\r
	/**\r
	 * @param {string} ty - type of event to listen for\r
	 * @param {function} handler - function to run when event is triggered\r
	 */\r
	addeventlistener({ selector, ty, value }) {\r
		const bc = new BroadcastChannel('listener_updates');\r
		let elem = document.querySelector(selector);\r
		document.querySelector(selector).addEventListener(ty, (e) => {\r
			// Because this cod eis in a blob:URL, we can't access the window object\r
			// directly. Instead, we post a message to the parent window\r
			bc.postMessage(window.render(e.target.value));\r
		});\r
	}\r
};\r
`;function _(a){let n,t;return{c(){n=new R(!1),t=w(),this.h()},l(e){n=T(e,!1),t=w(),this.h()},h(){n.a=t},m(e,r){n.m(a[0],e,r),g(e,t,r)},p(e,r){r&1&&n.p(e[0])},d(e){e&&(l(t),n.d())}}}function U(a){let n,t=a[0]&&_(a);return{c(){n=D("div"),t&&t.c()},l(e){n=H(e,"DIV",{});var r=P(n);t&&t.l(r),r.forEach(l)},m(e,r){g(e,n,r),t&&t.m(n,null),a[3](n)},p(e,[r]){e[0]?t?t.p(e,r):(t=_(e),t.c(),t.m(n,null)):t&&(t.d(1),t=null)},i:h,o:h,d(e){e&&l(n),t&&t.d(),a[3](null)}}}function O(a,n,t){let e,r,o;C(async()=>{const i=new BroadcastChannel("listener_updates"),{load:b}=await E(()=>import("../chunks/index.ec53056d.js"),["..\\chunks\\index.ec53056d.js","..\\chunks\\preload-helper.41c905a7.js"],import.meta.url);let v=await b(),B=await fetch(j).then(s=>s.arrayBuffer()),c="./importables.js";t(2,o=await v({wasmBytes:B,imprt:{map:{"component:cargo-comp":c},files:[[c,q]]}})),t(0,e=o.render("World")),window.render=o.render,i.onmessage=s=>{var d,p,f;let m=((f=(p=(d=new DOMParser().parseFromString(s.data||"","text/html"))==null?void 0:d.body)==null?void 0:p.firstElementChild)==null?void 0:f.id)||null;if(m){let u=document.getElementById(m);if(u){u.outerHTML=s.data;return}}t(0,e=s.data)}});function y(i){I[i?"unshift":"push"](()=>{r=i,t(1,r)})}return a.$$.update=()=>{a.$$.dirty&5&&e&&o&&(async()=>(await L(),o.listen()))()},[e,r,o,y]}class W extends M{constructor(n){super(),k(this,n,O,U,S,{})}}export{W as component};
