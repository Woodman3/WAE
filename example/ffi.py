from ctypes import * 
dl=WinDLL
dll=dl("./target/debug/wae.dll")
buffer = create_string_buffer(b'\0',100)
t=dll.init(b"./config")
if(t==0):
    ptr=dll.get_obs()
    cp = c_char_p(ptr) 
    # data=cp.value.decode()
    # print(data)
    
    dll.get_obs.restype=c_char_p
    data = dll.get_obs()
    print(ptr)
    print(data)
    
else:
    print("init error")