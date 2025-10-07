import os
import sys

from assassyn.ramulator2 import PyRamulator, Request

home = os.getenv('ASSASSYN_HOME', os.getcwd())
sim = PyRamulator(f"{home}/tools/c-ramulator2-wrapper/configs/example_config.yaml")

is_write = False
v = 0  # counter

for i in range(200):
    plused = v + 1
    we = v & 1
    re = not we
    raddr = v & 0xFF
    waddr = plused & 0xFF
    addr = waddr if is_write else raddr

    def callback(req: Request, i=i):  # capture i in closure    
       print(f"Cycle {i + 3 + (req.depart - req.arrive)}: "
              f"Request completed: {req.addr} the data is: {req.addr - 1}\n")

    ok = sim.send_request(addr, is_write, callback, i)
    write_success = "true" if ok else "false"
    if is_write:
        print(f"Cycle {i + 2}: Write request sent for address {addr}, "
              f"success or not (true or false){write_success}\n")

    is_write = not is_write
    sim.frontend_tick()
    sim.memory_system_tick()
    v = plused

sim.finish()