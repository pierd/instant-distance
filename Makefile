instant-distance-py/test/instant_distance.so: instant-distance-py/src/lib.rs
	cargo build --release
	([ -f target/release/libinstant_distance.dylib ] && cp target/release/libinstant_distance.dylib instant-distance-py/test/instant_distance.so) || \
	([ -f target/release/libinstant_distance.so ] && cp target/release/libinstant_distance.so instant-distance-py/test/instant_distance.so)

test-python: instant-distance-py/test/instant_distance.so
	PYTHONPATH=instant-distance-py/test/ python3 -m test

bench-python: instant-distance-py/test/instant_distance.so
	PYTHONPATH=instant-distance-py/test/ python3 -m timeit -n 10 -s 'import random, instant_distance; points = [[random.random() for _ in range(300)] for _ in range(1024)]' 'instant_distance.Hnsw.build(points, instant_distance.Config())'

clean:
	cargo clean
	rm -f instant-distance-py/test/instant_distance.so
