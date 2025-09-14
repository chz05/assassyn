# Install PyCDE

# TODO: Later add a flag to force CIRCT installation via source
pip install --user pycde --break-system-packages
if [ $? -eq 0 ]; then
  echo "CIRCT installed successfully via pip."
  return 0
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
python setup.py install --prefix="$DIST_DIR"

if [ $? -ne 0 ]; then
  echo "Failed to install PyCDE. Please check the installation output."
  cd $RESTORE
  return 1
fi

# Add the local installation to PYTHONPATH
# Find the actual Python site-packages directory
SITE_PACKAGES=$(find "$DIST_DIR/lib" -name "site-packages" -type d 2>/dev/null | head -n 1)

if [ -n "$SITE_PACKAGES" ]; then
  if [ -z "$PYTHONPATH" ]; then
    export PYTHONPATH="$SITE_PACKAGES"
  else
    export PYTHONPATH="$SITE_PACKAGES:$PYTHONPATH"
  fi
fi

cd $RESTORE

echo "PyCDE built and installed successfully to local directory."
return 0
