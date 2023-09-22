from typing import Dict, Union
import argparse
from pathlib import Path
import timeit

import numpy as np
from PIL import Image


DEFAULT_ROOT_DIR = "../assets/test_dataset"


def get_dataset_description(root_dir: str) -> Dict[str, Union[float, int]]:
    averages = []
    max_height = 0
    min_height = float("inf")
    max_length = 0
    min_length = float("inf")


    for entry in Path(root_dir).iterdir():
        if entry.is_file():
            image = np.array(Image.open(entry))
            pixels_averages = np.mean(image, axis=tuple(range(image.ndim-1)))
            averages.append(pixels_averages)

            if max_height < image.shape[1]:
                max_height = image.shape[1]

            if max_length < image.shape[0]:
                max_length = image.shape[0]

            if min_height > image.shape[1]:
                min_height = image.shape[1]

            if min_length > image.shape[0]:
                min_length = image.shape[0]

    averages = np.array(averages)
    dataset_average = np.mean(averages, axis=0)
    dataset_std = np.sqrt(np.mean((averages - dataset_average) ** 2, axis=0))


    return {
        "averages": dataset_average,
        "stds": dataset_std,
        "max_height": max_height,
        "min_height": min_height,
        "max_length": max_length,
        "min_length": min_length,
    }


def run_benchmark(args: argparse.Namespace, num_iter: int = 10):
    root_dir = args.root_dir
    total_time = timeit.timeit(lambda: get_dataset_description(root_dir), number=num_iter)
    print(f"Average running time: {total_time / num_iter} s")


def main(args: argparse.Namespace):
    dataset_description = get_dataset_description(args.root_dir)
    print(dataset_description)
    # run_benchmark(args)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--root-dir", type=str, default=DEFAULT_ROOT_DIR)
    args = parser.parse_args()
    main(args)
