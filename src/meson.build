concept_sources = [
  'main.vala',
  'application.vala',
  'window.vala',
]

concept_deps = [
  dependency('gtk4'),
  dependency('libadwaita-1'),
]

blueprints = custom_target('blueprints',
  input: files(
    'gtk/help-overlay.blp',
    'window.blp',
  ),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)

res = declare_dependency(
  sources: blueprints,
)

concept_deps += res
concept_sources += gnome.compile_resources('concept-resources',
    'concept.gresource.xml',
    c_name: 'concept',
    dependencies: blueprints,
  )
executable('concept', concept_sources,
  dependencies: concept_deps,
       install: true,
)
