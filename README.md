# Image dataset analyzer

Small rust crate that can extract several dataset wide descriptors.

---

### Example:
```
.\image_dataset_analyzer.exe --root-dir="D:\Storage\train_small" --timeit
Dataset description
    - pixels description:
        - red average: 124.63575
        - green average: 115.94765
        - blue average: 106.375145

        - red std: 29.338514
        - green std: 28.082455
        - blue std: 29.116316

    - images height min/max: 57/1050
    - images length min/max: 41/768
    - dataset size: 6000


Execution duration 2.8334343 seconds
Speed: 2117.5715 (images / second)
```

---
### Benchmark
The size of the dataset used: 500 images

- Python implementation (`benchmark/dataset_analyzer.py`): average running time: 2.19305682s
- Rust image_dataset_analyzer:
	- No parallelization
		- debug build: 20.156366 seconds
		- release build: 1.0892467 seconds
	- Rayon
		- debug build: 4.2945614 seconds
		- release build: 0.3085112 seconds
