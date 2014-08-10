import subprocess
import os
import shutil

include_dir = 'includes'
css_dir = 'css'
out_dir = 'out'

def ensure_dir(f):
    d = os.path.dirname(f)
    if not os.path.exists(d):
        os.makedirs(d)

def blast_away(d):
    if os.path.exists(d):
        for f in os.listdir(d):
            p = d+'/'+f
            if os.path.isdir(p):
                shutil.rmtree(d+'/'+f)
            else:
                os.remove(d+'/'+f)

def copy_file(folder, filename):
    in_file = folder + filename
    out_file = out_dir + '/' + in_file
    ensure_dir(out_file)
    subprocess.call(['cp', in_file, out_file])

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
files['css'] = ['style.css']
files['essays/aa_trees'] = ['index.md', 'bst.svg', 'header_aa.svg']
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

        if i.endswith('.md'):
            compile(folder, i, is_index)
        else:
            copy_file(folder, i)

# ugh
subprocess.call(['python', 'math_notes/build.py'])
