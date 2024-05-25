import json
# pdb.set_trace()
from ctypes import * 
dl=CDLL
dll=dl("./target/debug/wae.dll")
# dll=dl("./target/debug/libwae.so")
buffer = create_string_buffer(b'\0',100)
t=dll.init(b"./config")
if(t==0):
    dll.get_obs.restype =POINTER(c_char) 
    ptr = dll.get_obs()
    data = cast(ptr, c_char_p).value
    data = json.loads(data)
    dll.free_str(ptr)
    print(data["Map"])
    
else:
    print("init error")