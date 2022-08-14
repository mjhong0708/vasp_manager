#!/bin/bash
#SBATCH -e stderr-%j.log
#SBATCH -o stdout-%j.log
#SBATCH -p {{ partition }}
#SBATCH -N {{ num_nodes }}
#SBATCH -n {{ num_tasks }}

module purge
module add compiler/2022.1.0
module add mkl/2022.1.0
module add mpi/2021.6.0

VASP_VERSION={{ vasp_version }}
VASP={{ bin }}
VASP_BIN="$HOME/.local/VASP/bin/$VASP_VERSION/$VASP"

mpirun -np $SLURM_NTASKS $VASP_BIN
