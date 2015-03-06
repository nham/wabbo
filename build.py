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
                  '--highlight-style=zenburn'] + math)

    p = subprocess.call(pandoc_call)

files = {}
files[''] = ['index.md']
files['css'] = ['style.css']
files['written'] = ['bananas_string_matching_algorithms.md', 'small_period_pseudocode.png',
                      '8-queens.md',
                      ('fuzzy_string_matching_DP.md', 'math')]
files['essays/diff'] = ['diff_viz.html', 'diffdraw.js',
                                         'snap.svg-min.js',
                                         'jquery-2.1.1.min.js']

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
