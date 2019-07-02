import subprocess
import sys


def call(command):
    print(command)
    r = subprocess.call(command, shell=True)
    if r != 0:
        sys.exit(r)


def make():
    call('la echo unimplemented')


def test_pass():
    call('env RUST_MIN_STACK=134217728 cargo test test_vm')
    call('env RUST_MIN_STACK=134217728 cargo test test_state_pass')


def test_work():
    call('env RUST_MIN_STACK=134217728 RUST_LOG=evm=debug,state=debug,cita_vm=debug cargo test test_state_work -- --nocapture')


def test_work_pure():
    call('env RUST_MIN_STACK=134217728 cargo test test_state_work -- --nocapture')


def main():
    while True:
        call('la git pull')
        test_work()
        test_pass()


if __name__ == '__main__':
    main()
