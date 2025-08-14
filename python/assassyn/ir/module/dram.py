'''Memory module, a special and subclass of Module.'''

from .module import Module, combinational
from ..array import RegArray, Array
from ..block import Condition
from ..dtype import Bits
from ..expr import Bind
from ..value import Value
from ..expr import mem_write, send_read_request, has_mem_resp, mem_resp, wait_until, send_write_request
from ..expr import log

class DRAM(Module): # pylint: disable=too-many-instance-attributes
    '''The DRAM module.'''

    width: int  # Width of the memory in bits
    depth: int  # Depth of the memory in words
    init_file: str  # Path to initialization file
    payload: Array  # Array holding the memory contents
    we: Value  # Write enable signal
    re: Value  # Read enable signal
    addr: Value  # Address signal
    wdata: Value  # Write data signal
    bound: Bind  # Bind handle

    def __init__(self, width, depth, init_file):
        super().__init__(
            ports={} ,
        )  
        self.width = width
        self.depth = depth
        self.init_file = init_file
        self.payload = RegArray(Bits(width), depth, attr=[self])
        self.we = None
        self.re = None
        self.addr = None
        self.wdata = None
        self.bound = None

    @combinational
    def build(self, we, re, addr, wdata, handle_response): #pylint: disable=too-many-arguments
        '''The constructor for the DRAM module.

        # Arguments
        init_file: str: The file to initialize the memory.
        we: Value: The write enable signal.
        re: Value: The read enable signal.
        addr: Value: The address signal.
        wdata: Value: The write data signal.
        user: Module: The user module, it is required to have a rdata port.

        # Returns
        bound: Bind: The bound handle of the user module.
        '''
        self.we = we
        self.re = re
        self.addr = addr
        self.wdata = wdata

        # with Condition(we):
        #     mem_write(self.payload, addr, wdata)
        # with Condition(re):
        #     mem_read(self.payload, addr)
        #     self.bound = user.bind(rdata=self.payload[addr])
        # wait_until(self.we | has_mem_resp(self))
        kind_we = Bits(1)(0)
        kind_re = Bits(1)(0)
        write_success = Bits(1)(0)
        rdata = Bits(self.width)(0)
        succ = send_write_request(addr, self.we)

        kind_we = self.we
            
        with Condition(succ):
            mem_write(self.payload, addr, wdata)

        with Condition(self.re):
            send_read_request(addr)
        
        # if the request is not success, what we do for it. Do nothing now, we can change later.
        log('1111111111')
        has_resp = has_mem_resp(self)
        log('2222222222')
        mem_rdata = mem_resp(self)
        rdata = has_resp.select(mem_rdata, Bits(self.width)(0))
        log('3333333333')
        kind_re = has_resp.select(Bits(1)(1), Bits(1)(0))

        with Condition(self.we | has_resp):
            self.bound = handle_response.bind(kind_we = kind_we, 
                                              kind_re = kind_re, 
                                              write_success = write_success,
                                              data = rdata)

        return self.bound

    def __repr__(self):
        return f'DRAM(width={self.width}, depth={self.depth}, init_file={self.init_file})'