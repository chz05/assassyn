from assassyn.ramulator2.wrapper import PyRamulator, Request
import os

home = os.getenv('ASSASSYN_HOME', os.getcwd())
sim = PyRamulator(f"{home}/testbench/simulator/configs/example_config.yaml")

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
              f"Request completed: {req.addr} the data is: {req.addr - 1}")

    ok = sim.send_request(addr, is_write, callback, i)
    
    if is_write:
        print(f"Cycle {i + 2}: Write request sent for address {addr}, "
              f"success or not (true or false) {ok}")

    is_write = not is_write
    sim.frontend_tick()
    sim.memory_system_tick()
    v = plused

sim.finish()