python -m cProfile -o profiles/all_rust.prof python/all_rust.py
python -m cProfile -o profiles/rust_mod_11.prof python/rust_mod11.py
python -m cProfile -o profiles/rust_mod_11_and_date.prof python/rust_mod_and_date.py
python -m cProfile -o profiles/pure_python.prof python/pure_python.py