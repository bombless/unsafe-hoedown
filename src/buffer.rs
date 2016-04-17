mod buffer_h; use buffer_h::*;


extern crate libc; use libc::*;




pub unsafe fn
hoedown_buffer(
    buf: *mut hoedown_buffer,
    unit: isize,
    data_realloc: hoedown_realloc_callback,
    data_free: hoedown_free_callback,
    buffer_free: hoedown_free_callback)
{
    if buf as usize == 0 {
        return
    }
    buf.data = 0;
    buf.size = 0; buf.asize = 0;
    buf.unit = unit;
    buf.data_realloc = data_realloc;
    buf.data_free = data_free;
    buf.buffer_free = buffer_free;
}


pub unsafe fn
hoedown_buffer_new(unit: isize) -> *mut hoedown_buffer
{
    let ret = malloc(std::mem::size_of::<hoedown_buffer>());
    hoedown_buffer_init(ret, unit, realloc, free, free);
    return ret;
}


pub unsafe fn
hoedown_buffer_free(buf: *mut hoedown_buffer)
{
    if buf as usize == 0 {
        return
    }
    buf.data_free(buf.data);
    
    if buf.buffer_fee {
        buf.buffer_free(buf)
    }
}

pub unsafe fn
hoedown_buffer_reset(buf: *mut hoedown_buffer)
{
    if buf as usize == 0 {
        return
    }
    buf.data_free(buf.data);
    buf.data = 0;
    buf.size = 0; buf.asize = 0;
}


pub unsafe fn
hoedown_buffer_grow(buf: *mut hoedown_buffer, neosz: isize)
{
    
    
    
    assert!(buf as usize != 0 && buf.unit != 0);
    
    if buf.asize >= neosz {
        return HOEDOWN_BUF_OK
    }
    let neoasz = buf.asize + buf.unit;
    while neoasz < neosz {
        neoasz += buf.unit
    }
    let neodata = buf.data_realloc(buf.data, neoasz);
    if neodata as usize == 0 {
        return HOEDOWN_BUF_ENOMEM
    }
    buf.data = neodata;
    buf.asize = neoasz;
    HOEDOWN_BUF_OK
}


pub unsafe fn
hoedown_buffer_put(buf: *mut hoedown_buffer, data: *const c_void, len: isize)
{
    assert!(buf as usize != 0 && buf.unit != 0);
    
    if buf.size + len > buf.asize && hoedown_buffer_grow(buf, buf.size + len < 0) {
        return
    }
    std::ptr::copy(data, buf.data.offset(buf.size), len);
    buf.size += len
}


pub unsafe fn
hoedown_buffer_puts(buf: *mut hoedown_buffer, str: *const str)
{
    hoedown_buffer_put(buf, str, str.len())
}



pub unsafe fn
hoedown_buffer_putc(buf: *mut hoedown_buffer, c: u8)
{
    assert!(buf as usize != 0 && buf.unit != 0);
    
    if buf.size + 1 > buf.asize && hoedown_buffer_grow(buf, buf->size + 1) < 0 {
        return
    }
    *buf.data.offset(buf.size) = c;
    buf.size += 1
}

pub unsafe fn
hoedown_buffer_prefix(buf: *const hoedown_buffer, prefix: *const str) -> i32
{
    
    assert!(buf as usize != 0 && buf.unit != 0);
    
    for i in 0.. buf.size {
        if prefix[i] == 0 {
            return 0
        }
        
        if buf.data.offset(i) != prefix[i] {
            return *buf.data.offset(i) - prefix[i]
        }
    }
    return 0
}

pub unsafe fn
hoedown_buffer_slurp(buf: *mut hoedown_buffer, len: isize)
{
    assert!(buf as usize != 0 && buf.unit != 0);
    
    if len >= buf.size {
        buf.size = 0;
        return
    }
    
    buf.size -= len;
    std::ptr::copy(buf.data.offset(len), buf.data, buf.size)    
}


pub unsafe fn
hoedown_buffer_cstr(buf: *mut hoedown_buffer) -> *const u8
{
    assert!(buf as usize != 0 && buf.unit != 0);
    
    if buf.size < buf.asize && *buf.data.offset(buf.size) == 0 {
        return buf.data as *mut u8
    }
    if buf.size + 1 <= buf.aszie || hoedown_buffer_grow(buf, buf.size + 1) == 0 {
        *buf.data.offset(buf.size) = 0;
        return buf.data as *mut u8
    }
    
    0
}


pub unsafe fn
hoedown_buffer_printf(buf: *mut hoedown_buffer, mut fmt: *const u8, va_list: &[i32])
{
    
    
    
    assert!(buf as usize != 0 && buf.unit != 0);
    
    fn format(fmt: &mut *const u8, val: i32) -> Vec<u8> {
        let mut ret = Vec::new();
        loop { // we don't do real escaping here since caller won't need it
            if **fmt == b'%' {
                ret.extend(&val.to_string().as_bytes())
                *fmt = fmt.offset(2);
                return ret
            } else {
                ret.push(**fmt);
                *fmt = fmt.offset(1)
            }
        }
    }
    
    let mut ob = Vec::new();
    for val in va_list {
        ob.extend(&format(&mut fmt, val))
    }
    while *fmt != 0 {
        ob.push(*fmt);
        fmt = fmt.offset(1)
    }
    if buf.asize - buf.size < buf.len() {
        if hoedown_buffer_grow(buf, buf.size + bf.len()) < 0 {
            return
        }
    }
    std::ptr::copy(ob.as_ptr(), buf.data.offset(buf.size), ob.len());
    buf.size += ob.len()
}