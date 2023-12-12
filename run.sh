set -euxo pipefail

cargo build
cargo objcopy -- -O ihex none-fault.hex
sleep 1
probe-rs download --chip nrf52840 --format hex none-fault.hex
sleep 1
probe-rs reset --chip nrf52840
RTT=$(grep _SEGGER_RTT$ build.map | cut -d' ' -f1); echo $RTT >.rttlast
rtthost --chip nrf52840 --scan-region 0x$RTT
