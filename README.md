# rdk-rs

![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)
[![Latest version](https://img.shields.io/crates/v/rdk.svg)](https://crates.io/crates/rdk)
[![Conda CI](https://github.com/jcathalina/rdk-rs/actions/workflows/conda-ci.yml/badge.svg)](https://github.com/jcathalina/rdk-rs/actions/workflows/conda-ci.yml)

[Change Log](https://github.com/jcathalina/rdk-rs/blob/main/CHANGELOG.md)

## :rocket: Getting started
rdk-rs looks for dynamically linked (.so) files in the `$CONDA_PREFIX/lib/` directory within the conda installation of RDKit. To get up and running, simply run the following commands:
<br>
1. :snake: Install the latest [conda](https://github.com/conda-forge/miniforge) distribution for your OS (we recommend mini/mambaforge)
2. :package: Create a conda environment that includes a stable version of RDKit by running the commands below and activate it. Feel free to change the environment name `rdkit-env` to whatever you want.
```bash
mamba create -c conda-forge python=3.8 -n rdkit-env
conda activate rdkit-env
conda install -c conda-forge rdkit 
```
3. :link: Dynamically link the RDKit binaries we just downloaded, for example on Linux you would run the following:
```bash
export LD_LIBRARY_PATH=$CONDA_PREFIX/lib/
```
NOTE: The conda environment containing the RDKit installation must be active for $CONDA_PREFIX to contain the right path.