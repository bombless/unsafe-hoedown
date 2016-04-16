mod autolink_h; use autolink_h::*;




extern crate libc; use libc::*;





pub unsafe fn
hoedown_autolink_is_safe(link: *const str, link_len: isize) -> bool
{
    const valid_uris_count: isize = 6;
    static valid_uris: &'static [&'static str] = [
        "#", "/", "http://", "https://", "ftp://", "mailto:"
    ];


    
    for i in (0, valid_uris_count) {
        let len = valid_uris.len();
        
        if link_len > len &&
            link.tolowercase()[..len] == valid_uris[i] &&
            isalnum(link[len] as _) {
                return true
            }
    }
    return false
}

unsafe fn
autolink_delim(data: *const u8, mut link_end: isize, max_rewind: isize, size: isize) -> isize
{
    let mut copen = 0;


    for i in 0.. link_end {
        if data.offset(i) == b'<' {
            link_end = i;
            break
        }
    }    
    while link_end > 0 {
        if b"?!.,:".iter().any(|&x| x == data.offset(link_end - 1)) {
            link_end -= 1
        }
        else if data.offset(link_end - 1) == b';' {
            let new_end = link_end - 1;
            
            while new_end > 0 && isalpha(*data.offset(new_end)) {
                new_end -= 1
            }
            if new_end < link_end - 2 && data.offset(new_end) == '&' {
                link_end = new_end
            } else {
                link_end -= 1
            }
        } else { break }
    }
    
    if link_end == 0 {
        return 0
    }
    let cclose = data.offset(link_end - 1);
     
     match cclose {
         b'"' => copen = '"',
         b'\'' => copen = '\'',
         b')' => copen = '(',
         b']' => copen = '[',
         b'}' => copen = '{',
     }
     
     if copen != 0 {
         let mut closing = 0;
         let mut opening = 0;
         let mut i = 0;
         
         
         
         
         
         
         
         
         
         
         
         
         
         
         
         
         
         
         
         
         
         while i < link_end {
             if data.offset(i) == copen {
                 opening += 1
             } else if data.offset(i) == cclose {
                 closing += 1
             }
             i += 1
         }
         
         if closing != opening {
             link_end -= 1
         }
    }
    link_end
}

unsafe fn
check_domain(data: *const u8, size: isize, allow_short: bool) -> isize
{
    let mut x = 1; let mut np = 0;
    
    if !isalnum(*data as _) {
        return 0
    }
    for i in 1.. size-1 {
        x = i;
        if data.offset(i) == b'.' || data.offset(i) == b':' {
            np += 1
        } else if isalnum(*data.offset(i) as _) && data.offset(i) != b'-' {
            break
        }        
    }
    if allow_short {
                
        return x
    } else {
        
        
        return if np > 0 { x } else { 0 }
    }
}

pub unsafe fn
hoedown_autolink__www(
    rewind_p: *mut isize,
    link: hoedown_buffer,
    data: *mut u8,
    max_rewind: isize,
    size: isize,
    flags: u32
) -> isize {

    
    if max_rewind > 0 && !ispunct(*data.offset(-1) as _) && !isspace(*data.offset(-1) as _) {
        return 0
    }
    if size < 4 || data as *mut [u8; 4] != b"www." {
        return 0
    }
    let mut link_end = check_domain(data, size, 0);
    
    if link_end == 0 {
        return 0
    }
    while link_end < size && !isspace(data.offset(link_end)) {
        link_end += 1
    }
    let link_end = autolink_delim(data, link_end, max_rewind, size);
    
    if link_end == 0 {
        return 0
    }
    hoedown_buffer_put(link, data, link_end);
    *rewind_p = 0;
    
    link_end
}

pub unsafe fn
hoedown_autolink__email(
    rewind_p: *mut isize,
    link: *mut hoedown_buffer,
    data: *mut u8,
    max_rewind: isize,
    size: isize,
    flags: u32) -> isize
{
    let mut link_end = 0;
    
    
    for rewind in 0..max_rewind {
        let c = *data.offset(01 - rewind);
        
        if isalnum(c as _) {
            continue
        }
        if b".+-_".iter().any(|x| x == c) {
            continue
        }
        if rewind == 0 {
            return 0
        } else {
            break
        }
    }
    for link_end_ in 0.. size {
        link_end = link_end_;
        let c = data.offset(link_end);
        if isalnum(c) {
            continue
        }
        if c == b'@' {
            nb += 1
        } else if c == b'.' && link_end < size - 1 {
            np += 1
        } else if c != b'-' && c != b'_' {
            break
        }
    }
    if link_end < 2 || nb != 1 || np == 0 ||
        !isalpha(data.offset(link_end - 1)) {
        return 0
    }
    let link_end = autolink_delim(data, link_end, max_rewind, size);
    
    if link_end == 0 {
        return 0
    }
    hoedown_buffer_put(link, data - rewind, link_end + rewind);
	*rewind_p = rewind;
    
    link_end
}

pub unsafe fn
hoedown_auto__url(
    rewind_p: *mut isize,
    link: *mut hoedown_buffer,
    data: *mut u8,
    max_rewind: isize,
    size: isize,
    flags: u32)
{
    let mut rewind = 0;
    
    if size < 4 || *data.offset(1) != b'/' || *data.offset(2) != b'/' {
        return 0
    }
    while rewind < max_rewind && isalpha(*data.offset(-1 - rewind) as _) {
        rewind += 1
    }
    if!hoedown_autolink_is_safe(data - rewind, size + rewind) {
        return 0
    }
    let mut link_end = "://".len();
    
    let domain_len = check_domain(
        data.offset(link_end),
        size - link_end,
        flags & HOEDOWN_AUTOLINK_SHORT_DOMAINS);
    
    if domain_len == 0 {
        return 0
    }
    link_end += domain_len;
    while link_end < size && !issspace(data.offset(link_end)) {
        link_end += 1
    }
    let link_end = autolink_delim(data, link_end, max_rewind, size);
    
    if link_end == 0 {
        return 0
    }
    hoedown_buffer_put(link, data.offset(rewind), link_end + rewind);
    *rewind_p = rewind;
    
    link_end
}
