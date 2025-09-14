# Install PyCDE

# TODO: Later add a flag to force CIRCT installation via source
pip install --user pycde --break-system-packages
if [ $? -eq 0 ]; then
  echo "CIRCT installed successfully via pip."
  # Verify that PyCDE can be imported
  python3 -c "import pycde; print('PyCDE import verification: SUCCESS')"
  if [ $? -eq 0 ]; then
    echo "PyCDE import test passed."
    return 0
  else
    echo "WARNING: PyCDE installed via pip but import test failed."
    return 1
  fi
fi

RESTORE=`pwd`

echo "Failed to install CIRCT via pip. Fall back to building from source using PyCDE setup."
cd $ASSASSYN_HOME/3rd-party/circt/frontends/PyCDE

# Create local installation directory
mkdir -p "$DIST_DIR"

# Use the PyCDE setup.py to build just the PyCDE frontend
# Set ESI_RUNTIME=OFF to minimize external dependencies
CIRCT_DIRECTORY="$ASSASSYN_HOME/3rd-party/circt" CIRCT_EXTRA_CMAKE_ARGS="-DESI_RUNTIME=OFF" python setup.py build
DIST_DIR="`pwd`"/dist

if [ $? -ne 0 ]; then
  echo "Failed to build PyCDE from source. Please check the build output."
  cd $RESTORE
  return 1
fi

# Install the built package to local directory
python setup.py install

if [ $? -ne 0 ]; then
  echo "Failed to install PyCDE. Please check the installation output."
  cd $RESTORE
  return 1
fi

  
# Verify that PyCDE can be imported
python3 -c "import pycde; print('PyCDE import verification: SUCCESS')"
if [ $? -eq 0 ]; then
  echo "PyCDE import test passed."
else
  echo "WARNING: PyCDE built and installed but import test failed."
  cd $RESTORE
  return 1
fi

cd $RESTORE

echo "PyCDE built and installed successfully to local directory."
return 0
