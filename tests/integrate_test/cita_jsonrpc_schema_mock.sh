#!/usr/bin/env bash

set -e

SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
BINARY_DIR=${SOURCE_DIR}/target/install

################################################################################
echo -n "0) prepare  ...  "
. ${SOURCE_DIR}/tests/integrate_test/util.sh
cd ${BINARY_DIR}
echo "DONE"

################################################################################
echo -n "1) cleanup   ...  "
cleanup
echo "DONE"

################################################################################
echo -n "2) generate config  ...  "
${BINARY_DIR}/bin/admintool.sh > /dev/null 2>&1
echo "DONE"

################################################################################
echo -n "3) just start node0  ...  "
${BINARY_DIR}/bin/cita setup node0  > /dev/null
${BINARY_DIR}/bin/cita start node0 trace > /dev/null &
echo "DONE"

################################################################################
echo -n "4) generate mock data  ...  "
AMQP_URL=amqp://guest:guest@localhost/node0 \
        ${SOURCE_DIR}/target/debug/chain-executor-mock \
        -m ${SOURCE_DIR}/tests/interfaces/rpc/config/blockchain.yaml
echo "DONE"

################################################################################
echo "5) check mock data  ...  "
python2 ${SOURCE_DIR}/tests/interfaces/rpc/test_runner.py \
        --rpc-url http://127.0.0.1:1337 \
        --tests ${SOURCE_DIR}/tests/interfaces/rpc/tests/cita_blockNumber.json
echo "DONE"

################################################################################
echo -n "6) stop node0  ...  "
${BINARY_DIR}/bin/cita stop node0
echo "DONE"

################################################################################
echo -n "7) cleanup ... "
cleanup
echo "DONE"
