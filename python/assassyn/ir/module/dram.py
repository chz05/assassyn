'''Memory module, a special and subclass of Module.'''

from .downstream import Downstream
from .downstream import combinational as downstream_combinational
from .module import Module
from .module import combinational as module_combinational
from ..array import RegArray, Array
from ..block import Condition
from ..dtype import Bits
from ..expr import Bind
from ..value import Value
from ..expr import mem_write, send_read_request, has_mem_resp, send_write_request, use_dram

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
        super().__init__(ports={})
        self.width = width
        self.depth = depth
        self.init_file = init_file
        self.payload = RegArray(Bits(width), depth, attr=[self])
        self.we = None
        self.re = None
        self.addr = None
        self.wdata = None
        self.bound = None

    @module_combinational
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
        dram_handler = DRAM_handler(self.width, we, re, self.payload, addr, wdata, handle_response)
        self.bound = dram_handler.build()
        return self.bound

    def __repr__(self):
        return f'DRAM(width={self.width}, depth={self.depth}, init_file={self.init_file})'

class DRAM_handler(Downstream):
    '''The handler for the DRAM module.'''

    def __init__(self, width, we, re, payload, addr, wdata, handle_response):
        super().__init__()
        self.width = width
        self.we = we
        self.re = re
        self.payload = payload
        self.addr = addr
        self.wdata = wdata
        self.handle_response = handle_response
        self.bound = None

    @downstream_combinational
    def build(self):
        kind_we = Bits(1)(0)
        kind_re = Bits(1)(0)
        succ = send_write_request(self.addr, self.we)

        kind_we = self.we
        with Condition(succ):
            mem_write(self.payload, self.addr, self.wdata)
        with Condition(self.re):
            send_read_request(self.addr)
        has_resp = has_mem_resp(self)
        x = use_dram(self.handle_response.mem)
        x.fifo = self.handle_response.mem
        x.val = self.handle_response.mem
        self.bound = self.handle_response.bind()
        self.bound.pushes.append(x)
        kind_re = has_resp.select(Bits(1)(1), Bits(1)(0))
        with Condition(self.we | has_resp):
            self.bound.bind(kind_we = kind_we, 
                            kind_re = kind_re, 
                            write_success = succ)
        return self.bound
        
