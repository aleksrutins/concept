concept_sources = [
  'main.vala',
  'application.vala',
  'window.vala',
]

concept_deps = [
  dependency('gtk4'),
  dependency('libadwaita-1'),
]

concept_sources += gnome.compile_resources('concept-resources',
  'concept.gresource.xml',
  c_name: 'concept'
)

executable('concept', concept_sources,
  dependencies: concept_deps,
       install: true,
)
