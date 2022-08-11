# vasp_manager

Cli app for creating and submitting VASP job.

## Installation

```bash
git clone https://github.com/mjhong0708/vasp_manager.git
cargo install --path .
```

## Configuration

### POTCAR generation

Add `POTCAR_PATH_PREFIX` environment variable to set the path prefix of POTCAR files.

### VASP execution

To make this app work, your VASP binary must be stored like:

```bash
~/.local/VASP |
              |-- 5.4.4 - vasp_std, vasp_gam, ...
              |-- 6.3.1 - vasp_std, vasp_gam, ...
```

## Usage

```console
user@example.com:~$ vasp_manager init --help
vasp_manager-init 0.1.0

USAGE:
    vasp_manager init [OPTIONS]

OPTIONS:
        --bin <VASP_BIN>
            VASP binary to use. 'vasp_std', 'vasp_gam', ... [default: vasp_std]

    -d, --dir <DIR>
            The directory to create the job in [default: .]

    -h, --help
            Print help information

        --task <TASK>
            Task of the job. Default is 'relax'. `singlepoint` is also available [default: relax]

    -v, --vasp-version <VASP_VERSION>
            The version of VASP [default: 6.3.1]

    -V, --version
            Print version information

user@example.com:~$ vasp_manager create_job --help
vasp_manager-create_job 0.1.0

USAGE:
    vasp_manager create_job [OPTIONS]

OPTIONS:
    -d, --dir <DIR>    The directory to create the job in [default: .]
    -h, --help         Print help information
    -s, --submit       Whether submit job to slurm or not
    -V, --version      Print version information


```