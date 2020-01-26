from setuptools import setup

setup(
    name='emojis_cli',
    version='0.1',
    py_modules=['emojis_cli'],
    install_requires=[
        'click',
        'emojis'
    ],
    entry_points='''
        [console_scripts]
        emojis=emojis_cli:cli
    ''',
)