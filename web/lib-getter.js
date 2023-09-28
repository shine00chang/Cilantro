let lib_file = await fetch('../lib.wat')
  .then(res => res.text())
  .catch(console.error)

export function get_lib () {
  return lib_file;
}

