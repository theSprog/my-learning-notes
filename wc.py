#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import os
import sys


def count_words_in_markdown(markdown):
    text = markdown

    count_zh = 0
    for s in text:
        # 中文
        if '\u4e00' <= s <= '\u9fff':
            count_zh += 1
    return count_zh


def count_dir_word(dir):
    map = {}
    # 最为关键的函数, walk 会遍历文件夹下所有的文件, 返回 (路径，路径下文件夹，路径下文件)
    for dirpath, dirnames, filenames in os.walk(dir):
        # 由于我们只处理文件，所以不使用 dirnames
        for filename in filenames:
            if filename.endswith(".md"):
                path = os.path.join(dirpath, filename)  # 将路径和文件结合
                with open(path, 'r', encoding='utf8') as f:
                    count = count_words_in_markdown(f.read())
                    map[path] = count
    return map


def main():
    if sys.version_info < (3,):
        print('Python 3 is required. You are using Python 2. You should probably run this script as follows:')
        print('python3 mwc.py')
        sys.exit(1)

    if len(sys.argv) < 2:
        print('Provide the Markdown file to parse as first argument')
        sys.exit(1)

    if os.path.isfile(sys.argv[1]):
        with open(sys.argv[1], 'r', encoding='utf8') as f:
            print(count_words_in_markdown(f.read()))

    if os.path.isdir(sys.argv[1]):
        map = count_dir_word(sys.argv[1])
        map_sorted = sorted(map.items(), key=lambda x: x[1], reverse=False)
        print(map_sorted)
        print(sum([x[1] for x in map_sorted]))


if __name__ == '__main__':
    main()
