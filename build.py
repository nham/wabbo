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

def compile(folder, filename, is_index, use_math):
    in_file = folder + filename
    out_file = out_dir + '/' + folder + filename.replace('.md', '.html')

    ensure_dir(out_file)

    if not is_index:
        before_body = "before_body_page"
    else:
        before_body = "before_body"

    if use_math:
        math = ['--mathjax']
    else:
        math = []

    pandoc_call = (['pandoc', '-s', in_file,
                  '-t', 'html5',
                  '-o', out_file,
                  '--include-in-header', include_dir + '/header.html',
                  '--include-before-body', include_dir + '/'+before_body+'.html',
                  '--include-after-body', include_dir + '/footer.html',
                  '--smart',
                  '--highlight-style=espresso'] + math)

    p = subprocess.call(pandoc_call)

files = {}
files[''] = ['index.md']
files['css'] = ['style.css']
files['essays/aa_trees'] = [('index.md', 'math'), 'bst.svg', 'header_aa.svg']
files['blog'] = ['index.md']
files['blog/2014'] = ['11jul_a_bug_in_rusts_coherence_rules.md',
                      '16jul_for_loops_in_rust_no_std.md',
                      '03aug_09aug.md',
                      '22aug_on_bananas.md']

blast_away(out_dir)

for folder, lst in files.items():
    for i in lst:
        if folder != '':
            folder += '/'
            is_index = False
        else:
            is_index = True

        if isinstance(i, tuple):
            fname = i[0]
            mathjax = i[1] == 'math'
        else:
            fname = i
            mathjax = False

        if fname.endswith('.md'):
            compile(folder, fname, is_index, mathjax)
        else:
            copy_file(folder, fname)

# ugh
subprocess.call(['python', 'math_notes/build.py'])
