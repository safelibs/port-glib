project(
  'safe-glib-bootstrap',
  'c',
  version: '2.80.0',
  meson_version: '>=1.2.0',
)

cargo = find_program('cargo', required: true)
python3 = import('python').find_installation('python3', required: true)

message('Configuring the safe GLib bootstrap workspace')
message('tests=' + get_option('tests').to_string())
message('installed_tests=' + get_option('installed_tests').to_string())
message('introspection=' + get_option('introspection'))

run_target(
  'cargo-check',
  command: [cargo, 'check', '--workspace'],
)

run_target(
  'extract-abi',
  command: [python3, files('tools/extract_abi.py')],
)

