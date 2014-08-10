import subprocess
import os

include_dir = 'includes'
css_dir = 'css'
out_dir = 'out'

def ensure_dir(f):
    d = os.path.dirname(f)
    if not os.path.exists(d):
        os.makedirs(d)

def blast_away(d):
    if os.path.exists(d):
        subprocess.call(['rm', '-r', d])
        os.makedirs(d)

def compile(folder, filename, is_index):
    in_file = folder + filename
    out_file = out_dir + '/' + folder + filename.replace('.md', '.html')

    ensure_dir(out_file)

    if not is_index:
        before_body = "before_body_page"
    else:
        before_body = "before_body"

    pandoc_call = (['pandoc', '-s', in_file,
                  '-t', 'html5',
                  '-o', out_file,
                  '--include-in-header', include_dir + '/header.html',
                  '--include-before-body', include_dir + '/'+before_body+'.html',
                  '--include-after-body', include_dir + '/footer.html',
                  '--smart'])

    p = subprocess.call(pandoc_call)

files = {}
files[''] = ['index.md']
files['blog'] = ['index.md']
files['blog/2014'] = ['22jun_28jun.md', '06jul_12jul.md', '13jul_20jul.md', '03aug_09aug.md']

blast_away(out_dir)

for folder, lst in files.items():
    for i in lst:
        if folder != '':
            folder += '/'
            is_index = False
        else:
            is_index = True

        compile(folder, i, is_index)

subprocess.call(['cp', '-r', css_dir, out_dir])

# ugh
subprocess.call(['python', 'math_notes/build.py'])
