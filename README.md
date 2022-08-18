# vasp_manager

Cli app for creating and submitting VASP job.

## Installation

Compile and install with:


```bash
git clone https://github.com/mjhong0708/vasp_manager.git
cargo install --path .
```

Or, download binary in release tab.

## Setup

The following steps are required to setup 1vasp_manager`:

1. Set environment variable `POTCAR_PREFIX_PATH`
2. Setup VASP executable binaries

### Set environment variable `POTCAR_PREFIX_PATH`

The environment variable `POTCAR_PREFIX_PATH` is used to locate POTCAR files and generating POTCAR automatically with respect to given POSCAR.
VASP POTCAR files are usually locates as following example:

```
POTCAR_PBE
├── Ac
├── Ag 
...
├── Zn_sv_GW
├── Zr_sv
└── Zr_sv_GW
```

Then, the environment variable `POTCAR_PREFIX_PATH` should be set to the
full path of 'POTCAR_PBE' directory, for example, $HOME/.local/POTCAR_PBE.
In this case, add the following line to your ~/.bashrc or ~/.zshrc:

```bash
export POTCAR_PREFIX_PATH=$HOME/.local/POTCAR_PBE
```

and restart your shell (or run `source ~/.bashrc`).

### Setup VASP executable binaries

vasp_manger expects VASP executable binaries to be located in $HOME/.local/VASP/bin following hierarchy:

```
$HOME/.local/VASP/bin
├── 5.4.4
│   ├── vasp_std
│   ├── vasp_gam
│   ├── vasp_vtst_std
│   └── vasp_vtst_gam
├── 6.3.1
│   ├── vasp_std
│   ├── vasp_gam
│   ├── vasp_vtst_std
│   └── vasp_vtst_gam
...
```

So, in order to get vasp_manager working, you need to setup your VASP executable binaries like above.
If you cannot copy or move binaries because of permission issue, you can just create symbolic link to them.


## Usage

Run `vasp_manager` with `--help` option to see usage.