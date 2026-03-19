from setuptools import Extension, setup
from Cython.Build import cythonize


extensions = [
    Extension(
        name="solver",
        sources=["solver.pyx"],
    )
]


setup(
    name="advanced24solver",
    ext_modules=cythonize(
        extensions,
        compiler_directives={"language_level": "3"},
        annotate=False,
    ),
)
