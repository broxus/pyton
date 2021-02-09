## PyTON
Python bindings to tonlib

### Build

```bash
cargo build --release
# copy without `lib` prefix
cp /target/release/libpyton.so /path/to/project/python.so
```

### Example

```python
from pyton import get_contract_info

with open("Test.tvc", 'rb') as file:
    tvc = file.read()

    info = get_contract_info(tvc)
    print(f"Code hash: {info.code_hash}")
    print(f"Data hash: {info.data_hash}")
```
