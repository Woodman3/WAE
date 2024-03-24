from ctypes import * 
dl=cdll.LoadLibrary
dll=dl("./target/debug/wae.dll")
buffer = create_string_buffer(b'\0',100)
t=dll.init(b"./config")
if(t==0):
    # ptr=dll.get_obs()
    # cp = c_char_p(ptr) 
    # data=cp.value
    # print(data)
    
    dll.get_obs.restype=c_char_p
    ptr = dll.get_obs()
    print(ptr)
    # dll.free_str.argtypes=[c_char_p]
    # r=dll.free_str(ptr)
    print(r)