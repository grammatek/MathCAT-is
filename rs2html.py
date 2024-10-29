"""
Read a MathCAT test file in Rust and extract the MathML elements.
Write a html file that lists all extracted <math></math> elements in an numbered list.
"""
import os
import re

math_regex = re.compile(r'<math>.+?</math>', re.DOTALL)

testfile = ''
category = ''

html_start = f'<!doctype html> \
<html lang="en-US"> \
  <head> \
    <meta charset="UTF-8" /> \
    <title>MathML</title> \
  </head> \
  <body> \
    <h1>MathML - {testfile} - {category}</h1> \
<ol>'

html_end = '</ol> \
</body> \
</html>'


def collect_math_elements(content):
    matches = math_regex.findall(content)

    return matches


def write_html(elements, in_filename, prefix):
    testfile = in_filename[:-3]
    category = prefix

    if category:
        out_path = f'display_tests/{category}/{testfile}.html'
    else:
        out_path = f'display_tests/{testfile}.html'

    with open(out_path, 'w') as f:
        f.write(html_start)
        for elem in elements:
            f.write('<li>\n</br>\n<p>\n')
            f.write(elem)
            f.write('</br></br>\n</p>\n</li>\n<hr>')
        f.write(html_end)

def main():

    for dirpath, dirnames, filenames in os.walk('tests/Languages/is'):
        for filename in filenames:
            file_path = os.path.join(dirpath, filename)
            with open (file_path) as f:
                content = f.read()
                math_elements = collect_math_elements(content)
            prefix = ''
            if 'ClearSpeak' in file_path:
                prefix = 'ClearSpeak'
            elif 'SimpleSpeak' in file_path:
                prefix = 'SimpleSpeak'
            write_html(math_elements, filename, prefix)


if __name__ == '__main__':
    main()