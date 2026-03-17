#!/bin/bash
# Deploy script for librga-rs to RK3566 device

set -e

# Device configuration
DEVICE_IP="192.168.2.202"
DEVICE_USER="radxa"
DEVICE_PASS="radxa"
DEVICE_DIR="/home/radxa/rga-rs"

# Build target
TARGET="aarch64-unknown-linux-gnu"

echo "=== librga-rs Deployment Script ==="
echo "Target: ${DEVICE_USER}@${DEVICE_IP}:${DEVICE_DIR}"
echo ""

# Build release binary
echo "[1/4] Building release binary for ${TARGET}..."
cross build --release --target ${TARGET} --bin test_runner

# Build examples
echo "[2/4] Building examples..."
cross build --release --target ${TARGET} --examples

echo ""
echo "[3/4] Creating deployment package..."

# Create temp directory for deployment
DEPLOY_DIR=$(mktemp -d)
trap "rm -rf ${DEPLOY_DIR}" EXIT

# Copy binaries
cp target/${TARGET}/release/test_runner ${DEPLOY_DIR}/
cp target/${TARGET}/release/examples/basic_resize ${DEPLOY_DIR}/ 2>/dev/null || true
cp target/${TARGET}/release/examples/format_convert ${DEPLOY_DIR}/ 2>/dev/null || true
cp target/${TARGET}/release/examples/blend_demo ${DEPLOY_DIR}/ 2>/dev/null || true
cp target/${TARGET}/release/examples/benchmark ${DEPLOY_DIR}/ 2>/dev/null || true
cp target/${TARGET}/release/examples/image_resize ${DEPLOY_DIR}/ 2>/dev/null || true
cp examples/test.jpg ${DEPLOY_DIR}/ 2>/dev/null || true

# Copy library
cp libs/Linux/gcc-aarch64/librga.so ${DEPLOY_DIR}/

# Create run script
cat > ${DEPLOY_DIR}/run_test.sh << 'EOF'
#!/bin/bash
export LD_LIBRARY_PATH=".:${LD_LIBRARY_PATH}"
echo "Running librga test runner..."
./test_runner "$@"
EOF
chmod +x ${DEPLOY_DIR}/run_test.sh

echo "[4/4] Deploying to device..."

# Create remote directory
sshpass -p "${DEVICE_PASS}" ssh -o StrictHostKeyChecking=no ${DEVICE_USER}@${DEVICE_IP} "mkdir -p ${DEVICE_DIR}"

# Copy files
sshpass -p "${DEVICE_PASS}" scp -o StrictHostKeyChecking=no ${DEPLOY_DIR}/* ${DEVICE_USER}@${DEVICE_IP}:${DEVICE_DIR}/

echo ""
echo "=== Deployment Complete ==="
echo "Files deployed to: ${DEVICE_USER}@${DEVICE_IP}:${DEVICE_DIR}"
echo ""
echo "To run tests on the device:"
echo "  ssh ${DEVICE_USER}@${DEVICE_IP} 'cd ${DEVICE_DIR} && ./run_test.sh'"
echo ""
echo "Or manually:"
echo "  ssh ${DEVICE_USER}@${DEVICE_IP}"
echo "  cd ${DEVICE_DIR}"
echo "  export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH"
echo "  ./test_runner"
echo ""
