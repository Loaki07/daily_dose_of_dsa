# DSA - Python Solutions 

## Installation

Instructions for setting up the project.

```bash
conda create -n dsa python=3.10

conda activate dsa

pip install -r requirements.txt

conda deactivate
```

```bash
# install poetry
curl -sSL https://install.python-poetry.org | python3 -

# add dependencies
poetry add <package_name>

# install dependencies
poetry install

# build your project for distribution
poetry build
```


## Run

```bash
# for all the tests
pytest

# for a single file
pytest src/hello_world.py

# clean all the cache
python clean_python_cache.py
```

## Format

```bash
# format entire python project
black .

poetry run autopep8 --max-line-length=30 --in-place --recursive . --verbose
```