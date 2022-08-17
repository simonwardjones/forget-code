# Environments in python

For a great covering of virtual environments checkout the official [documentation](https://docs.python.org/3/tutorial/venv.html)

## Using pip, venv and requirements.txt

Once we have installed a version of python on our machine we have a `python` executable, together with `pip` executable and along side this a `site_packages` folder - which is a special folder where the packages are saved when we install them.

Different projects may require different versions of the same package and so we need to provide separate isolated environment for each project. In order to achieve this with python we use a virtual environment for each project. The virtual environment creates a copy of python, pip and has its own `site_packages` folder where it will store the packages we require for the project.

There are various tools that can help you to manage the creation of the environment and also a configuration file which will live in your project to list the dependencies that need to be installed into the virtual environment.

For the example below let's say we are working on a new project called `zebra`. We first make the new project folder - the code below makes the folder `~/learning/zebra` and moves to the folder.

```zsh
cd ~
mkdir -p learning/zebra
cd learning/zebra
```

Now we have the project folder let's start by manually creating a virtual environment using the python we have already installed:

```bash
python -m venv venv
```

The above command creates the virtual environment within the folder in the `venv` folder of our `zebra` project, specifically here:  `~/learning/zebra/venv`.

Now that we have created the virtual environment we want to tell the command line to use it! To do that one "activates" the environment with the following command:

```zsh
source venv/bin/activate
```

You can also replace the `source` word with a dot
```zsh
. venv/bin/activate
```

Now that we have activated the environment we can verify the python command is pointing to our new copy in the `venv` folder by running:

```zsh
which python
```

We can deactivate the environment at anytime by running the deactivate command

```zsh
deactivate
```

However now the python and pip commands would be our global versions and we don't want to use these so let's ensure our virtual environment is activated.

```zsh
. venv/bin/activate
```

Now we can install some dependencies, maybe our zebra project requires `pandas` and `plotly`.

We could install them as follows:
```zsh
pip install pandas plotly
```

This would install them into the virtual environment and add them to the special `site_packages` location but we would have no record of what we installed.

Later when we push the `zebra` project to github to share it with the world we want other users to be able to create their own virtual environment with the same dependencies. The easiest way to do this is to create a `requirements.txt` file in the `zebra` folder and list the dependencies in here:

> **~/learning/zebra/requirements.txt**
> ```txt
> pandas
> plotly
> ```

Now with the environment activated you can install all the dependencies from the file using

```zsh
pip install -r requirements.txt
```

```zsh
deactivate
```

## Using pipenv and a Pipfile

In the example above we had to create the virtual environment and then manage the dependencies as a separate step. There are tools out there that help us to manage the virtual environment and the dependencies in fewer commands.

Lets start a fresh project `lion` and move into the project.

```zsh
cd ~
mkdir -p learning/lion
cd learning/lion
```

We want to use `pipenv` tool, so first we install it into our global python
```zsh
pip install pipenv
```

Now instead of creating the virtualenv with `python -m venv venv` we simply run the following command:

```zsh
pipenv install
```

This command will create a virtual environment for the project. Unlike when we used `venv` above the environment is not store in the project - instead it is stored in a specific folder pipenv maintains; on a mac I it is `~/.local/share/virtualenvs`. As well as creating the virtualenv, pipenv created a special file `Pipfile` with the following contents:

> **~/learning/lion/Pipfile**
> ```toml
> [[source]]
> url = "https://pypi.org/simple"
> verify_ssl = true
> name = "pypi"
> 
> [packages]
> 
> [dev-packages]
> 
> [requires]
> python_version = "3.10"
> ```

This file is used to list the dependencies for a project managed with pipenv much like the `requirements.txt` did previously.

We can update the file with some dependencies:

> **~/learning/lion/Pipfile**
> ```toml
> [[source]]
> url = "https://pypi.org/simple"
> verify_ssl = true
> name = "pypi"
> 
> [packages]
> plotly = "*"
> pandas = "*"
> 
> [dev-packages]
> 
> [requires]
> python_version = "3.10"
> ```

Now when we run `pipenv install` again and it will install the packages into the virtual environment. Much like when we created the env with `venv` we still need to activate the environment ahead of running `python example.py` (assuming you made a file called `example.py`). We do that using the following command:

```zsh
pipenv shell
```

Again you can verify you have activated the environment by running:
```zsh
which python
```

```zsh
deactivate
```

There's quite a bit more to pipenv so check it out here: https://pipenv.pypa.io/en/latest/

## Using poetry and pyproject.toml

Poetry is similar to `pipenv` in many ways!

Lets start a fresh project `cat`. The easiest way to do this is to let poetry do it for us:

```zsh
poetry new cat
cd cat
```

This would have set us up with the following project structure:
```
.
├── README.rst
├── cat
│   └── __init__.py
├── pyproject.toml
└── tests
    ├── __init__.py
    └── test_cat.py

2 directories, 5 files
```

Importantly the `pyproject.toml` file will store project configuration including the requirements:

> **~/learning/cat/pyproject.tom**
> ```toml
> [tool.poetry]
> name = "cat"
> version = "0.1.0"
> description = ""
> authors = ["simonwardjones <simonwardjones@yahoo.co.uk>"]
> 
> [tool.poetry.dependencies]
> python = "^3.10"
> 
> [tool.poetry.dev-dependencies]
> pytest = "^5.2"
> 
> [build-system]
> requires = ["poetry-core>=1.0.0"]
> build-backend = "poetry.core.masonry.api"
> 
> ```

Now the project is set up we can create the virtual environment, and install the dependencies in the `pyproject.toml` file by running:

```zsh
poetry install
```

Similar to `pipenv`, `poetry` created the virtual environment in a special folder, with poetry on my mac it is in here: `~/Library/Caches/pypoetry/virtualenvs`.

Finally we want to activate the environment, this time with
```zsh
poetry shell
```

```zsh
deactivate
```



### A note on using pycharm and other ides

Many of the IDEs allow you to specify your "python interpreter". This means telling the IDE which version of python to use when running the script (often via a play button). You will often want to tell the IDE to use the python version in our virtual environment.