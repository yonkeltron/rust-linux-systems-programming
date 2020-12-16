
asciidoc_files = FileList['src/*.adoc']

rule ".html" => ".adoc" do |t|
  sh "asciidoctor #{t.source} -o #{t.name}"
end

desc "Render slides using AsciiDoctor"
task :html => asciidoc_files.ext('.html')

task default: :html

puts asciidoc_files