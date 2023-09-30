/* This is a JS API that is exposed to Cilantro
 * Cilantro uses this API to fetch the stdlib file
 */

let lib_file = await fetch('../lib.wat')
  .then(res => res.text())
  .catch(console.error)

export function get_lib () {
  return lib_file;
}

