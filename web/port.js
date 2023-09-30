import init, { compile as cilantro_compile, tobytes as cilantro_tobytes } from '../js-pkg/cilantro.js';


const output_el = document.querySelector('#output-div');

console.log('== Initializing.. ==');
await init();
console.log('== Done! ==');


/* Calls Cilantro to compile source into { WAT, WASM } 
 * This function contains all Cilantro-relevant calls.
 */
export async function compile (source) 
{
  console.log('== Compiling.. ==');
  let wat, wasm;
  try {
    wat  = cilantro_compile(source);
    wasm = cilantro_tobytes(wat);
  } catch (err) {
    console.error('== Error at Compilation ==')
    output_el.innerText += '== Error at Compilation ==';
    return undefined;
  }

  console.log('== Done! ==');

  return { wat, wasm };
}


/* Sets polyfill, instantiates & executes WASM module.
 * Sets the global state `program_instance` for polyfill
 * functions to gain access to instance-bound objects (linear memory).
 */
let program_instance;
export async function run_wasm (wasm) 
{
  if (wasm === undefined) return console.error('run_wasm() received no input');


  console.log('== Running WASI ==');
  const importObject = { wasi_unstable: wasi_polyfill };

  const headers  = new Headers({ 'Content-Type': 'application/wasm' });
  const response = new Response(wasm.buffer, { headers });
  const result   = await WebAssembly.instantiateStreaming(response, importObject)
    .catch(err => {
      output_el.innerText += '== WASM Instantiation Error ==\n';
      output_el.innerText += err;
    });
  if (result == undefined) return;

  program_instance = result.instance;
  try {
    program_instance.exports._start();
  } catch(e) {
    output_el.innerText += "== Runtime Error ==";
  }
}


/* Polyfill Object for WASI interface */
const wasi_polyfill = {
  fd_write
};


/* WASI Polyfill function - prints to '#console' */
function fd_write(fd, iovs, iovsLen, nwritten) 
{
  let view = new DataView(program_instance.exports.memory.buffer);

  let sum = 0;

  let buffers = Array.from({ length: iovsLen }, function (_, i) {
    let ptr = iovs + i * 8;
    let buf = view.getUint32(ptr, true);
    let bufLen = view.getUint32(ptr + 4, true);

    sum += bufLen;

    return new Uint8Array(program_instance.exports.memory.buffer, buf, bufLen);
  });

  let bufferBytes = new Uint8Array(sum);
  let i = 0;
  buffers.forEach(buffer => {
    for (let j = 0; j < buffer.length; j++) {
      if (buffer[j] == 10) buffer[j] = 13;
      bufferBytes[i++] = buffer[j];
    }
  });

  output_el.innerText += new TextDecoder("utf-8").decode(bufferBytes);

  view.setUint32(nwritten, sum, true);

  return 0;
}
