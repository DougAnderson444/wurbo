// This code must be stand alone and 100% resolved into this single file
// as it gets stringified and rolled up into the bundle of BlobURL code that loads the wasm
export function buildCodeString(namespace) {
	return `
      const bc = new BroadcastChannel('${namespace}');
      export function addeventlistener({ selector, ty }) {
        document.querySelector(selector).addEventListener(ty, (e) => {
          let ctx = {
            tag: e.target.name,
            val: {
              value: e.target.value,
            }
          };
          bc.postMessage(window.${namespace}.render(ctx, e.target.dataset.contextTarget));
        });
      }`;
}
