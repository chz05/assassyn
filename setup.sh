# NOTE: This script should be sourced by ZSH! O.w. the directory behaviors will be wrong!

# Check for --no-verilator flag
NO_VERILATOR=false
for arg in "$@"; do
  if [ "$arg" = "--no-verilator" ]; then
    NO_VERILATOR=true
    break
  fi
done

# Restore the original directory
RESTORE_DIR=`pwd`

# Go to the setup.sh directory
cd `dirname $0`

# Use the repository path to set the PYTHONPATH and ASSASSYN_HOME
REPO_PATH=`git rev-parse --show-toplevel`

which sccache > /dev/null 2>&1

if [ $? -eq 0 ]; then
  echo "Setting up SCCACHE to `which sccache`"
  export RUSTC_WRAPPER=`which sccache`
else
  echo "No sccache found! Skip!"
fi

echo "Adding $REPO_PATH/python to PYTHONPATH"
export PYTHONPATH=$REPO_PATH/python:$PYTHONPATH

echo "Setting ASSASSYN_HOME to $REPO_PATH"
export ASSASSYN_HOME=$REPO_PATH

if [ -d "$REPO_PATH/3rd-party/circt/frontends/PyCDE/dist/lib" ]; then
  echo "Adding PyCDE to PYTHONPATH."
  export PYTHONPATH="$REPO_PATH/3rd-party/circt/frontends/PyCDE/dist/lib:$PYTHONPATH"
fi

if [ "$NO_VERILATOR" = false ]; then
  echo "In-repo verilator found, setting VERILATOR_ROOT to $REPO_PATH/verilator"
  export VERILATOR_ROOT=$REPO_PATH/3rd-party/verilator
  export PATH=$VERILATOR_ROOT/bin:$PATH
else
  echo "Verilator is disabled by --no-verilator flag"
fi

# Export library paths based on the operating system
OS=$(uname -s)
if [ "$OS" = "Darwin" ]; then
  # For macOS, use DYLD_LIBRARY_PATH
  echo "Adding $REPO_PATH/3rd-party/ramulator2 to DYLD_LIBRARY_PATH"
  export DYLD_LIBRARY_PATH=$REPO_PATH/3rd-party/ramulator2:$DYLD_LIBRARY_PATH
  echo "Adding $REPO_PATH/testbench/simulator/build/lib to DYLD_LIBRARY_PATH"
  export DYLD_LIBRARY_PATH=$REPO_PATH/testbench/simulator/build/lib:$DYLD_LIBRARY_PATH
else
  # For Linux, use LD_LIBRARY_PATH
  echo "Adding $REPO_PATH/3rd-party/ramulator2 to LD_LIBRARY_PATH"
  export LD_LIBRARY_PATH=$REPO_PATH/3rd-party/ramulator2:$LD_LIBRARY_PATH
  echo "Adding $REPO_PATH/testbench/simulator/build/lib to LD_LIBRARY_PATH"
  export LD_LIBRARY_PATH=$REPO_PATH/testbench/simulator/build/lib:$LD_LIBRARY_PATH
fi

# Go back to the original directory
cd $RESTORE_DIR
