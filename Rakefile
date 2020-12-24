
asciidoc_files = FileList['src/*.adoc']

rule ".html" => ".adoc" do |t|
  sh "asciidoctor #{t.source} -D public"
end

desc "Render slides using AsciiDoctor"
task html: asciidoc_files.ext('.html')

task :setup do
  sh 'mkdir -p public'
end

task default: [:setup, :html]

puts asciidoc_files