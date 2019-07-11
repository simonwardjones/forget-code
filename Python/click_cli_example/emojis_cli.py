import emojis
import click
from emojis.db.db import EMOJI_DB

# get both help strings
CONTEXT_SETTINGS = dict(help_option_names=['-h', '--help'])


@click.group(context_settings=CONTEXT_SETTINGS,)
def cli():
    pass


@cli.command(
    short_help='Encode emojis in strings 😃')
@click.argument(
    'argument',
    nargs=-1,
    required=True)
@click.option(
    '--output',
    '-o',
    help='Optional output filename.',
    type=click.File('w'))
@click.option(
    '--quiet',
    '-q',
    help='Don\'t print result to stdout.',
    is_flag=True)
@click.option(
    '--n-repeat',
    '-n',
    help='Repeat n times',
    default=1)
def encode(argument, output=None, quiet=None, n_repeat=None):
    """Encode emojis in strings 😃

    \b
    minimal example:
    >>> emjois :wave: -3
    >>> 👋👋👋
    """
    processed_input = emojis.encode(' '.join(argument)) * n_repeat
    if output:
        click.echo(processed_input, file=output)
    if not quiet:
        click.echo(processed_input)


@cli.command(
    short_help='List emojis with names')
@click.option(
    '--category',
    '-c',
    help='Filter the empjis by category')
def list(category=None):
    """List emojis with names

    \b
    optionaly specify category from:
        'activities',
        'animals_and_nature',
        'llags',
        'food_and_drink',
        'objects',
        'people_and_body',
        'smilies',
        'symbols',
        'travel'
    """
    lookup_category = {
        'activities': 'Activities',
        'animals_and_nature': 'Animals & Nature',
        'flags': 'Flags',
        'food_and_drink': 'Food & Drink',
        'objects': 'Objects',
        'people_and_body': 'People & Body',
        'smilies': 'Smileys & Emotion',
        'symbols': 'Symbols',
        'travel': 'Travel & Places'}
    category = lookup_category.get(category)
    if category:
        emojis_list = [e
                       for e in EMOJI_DB if e.category == category]
    else:
        emojis_list = EMOJI_DB
    output = '\n'.join('{emoji} - {emoji_name}'.format(
        emoji=emoji.emoji,
        emoji_name=",".join(emoji.aliases))
        for emoji in emojis_list)
    click.echo_via_pager(output)


if __name__ == '__main__':
    cli()
