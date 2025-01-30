# Bigbytes Python Binding

This crate intends to build a native python binding.

## Installation

```bash
pip install bigbytes
```

## Usage

### Basic:

```python
from bigbytes import SessionContext

ctx = SessionContext()

df = ctx.sql("select number, number + 1, number::String as number_p_1 from numbers(8)")

# convert to pyarrow
df.to_py_arrow()

# convert to pandas
df.to_pandas()

```

### Register external table:

***supported functions:***
- register_parquet
- register_ndjson
- register_csv
- register_tsv

```python

ctx.register_parquet("pa", "/home/sundy/dataset/hits_p/", pattern = ".*.parquet")
ctx.sql("select * from pa limit 10").collect()
```


### Tenant separation:

```python
ctx = SessionContext(tenant = "a")
```


## Development

Setup virtualenv:

```shell
python -m venv .venv
```

Activate venv:

```shell
source .venv/bin/activate
````

Install `maturin`:

```shell
pip install "maturin[patchelf]"
```

Build bindings:

```shell
maturin develop
```

Run tests:

```shell
maturin develop -E test
```

Build API docs:

```shell
maturin develop -E docs
pdoc bigbytes
```

## Storage configuration

- Meta Storage directory(Catalogs, Databases, Tables, Partitions, etc.): `./.bigbytes/_meta`
- Data Storage directory: `./.bigbytes/_data`
- Cache Storage directory: `./.bigbytes/_cache`
- Logs directory: `./.bigbytes/logs`

## More

Bigbytes python api is inspired by [arrow-datafusion-python](https://github.com/apache/arrow-datafusion-python), thanks for their great work.
