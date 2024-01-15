// This code must be stand alone and 100% resolved into this single file
// as it gets stringified and rolled up into the bundle of BlobURL code that loads the wasm
export function buildCodeString(namespace) {
	return `
      const bc = new BroadcastChannel('${namespace}');
      export function addeventlistener({ selector, ty }) {
        document.querySelector(selector).addEventListener(ty, (e) => {

          // detect if form event
          if(e.target.closest('form')) {
            e.preventDefault();
          }

          let tag  = e.target.dataset.contextName || e.target.name;

          console.log('tag', tag, e.target.dataset.contextName);

          if(!tag) { 
            console.warn('No name or data-context-name found for event: ', e.target);
            return;
          }

          console.log(typeof JSON.parse(e.target.dataset.contextValue) === 'object', {ctxVal: JSON.parse(e.target.dataset.contextValue)});

          let ctx = { 
            tag, 
            val: Object.assign(
                  {}, 
                  typeof JSON.parse(e.target.dataset.contextValue) === 'object' ? JSON.parse(e.target.dataset.contextValue) : {}, 
                  { value: e.target.value }) 
          };

          let el = e.target.closest('[data-slot]');
          if(el) {
            ctx = { tag: el.dataset.slot, val: ctx };
            el = el.closest('[data-slot]');
          }

          console.log({ctx});
          let rendered = window.${namespace}.render(ctx); 
          bc.postMessage(rendered);
        });
      }
`;
}
