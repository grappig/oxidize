use std::{io, ptr};

pub fn unshare() -> io::Result<()> {
    let flags = libc::CLONE_NEWNS | libc::CLONE_NEWUTS | libc::CLONE_NEWIPC | libc::CLONE_NEWPID;

    if unsafe { libc::unshare(flags) } != 0 {
        return Err(io::Error::last_os_error());
    }

    make_mounts_private()
}

fn make_mounts_private() -> io::Result<()> {
    let result = unsafe {
        libc::mount(
            ptr::null(),
            c"/".as_ptr(),
            ptr::null(),
            libc::MS_REC | libc::MS_PRIVATE,
            ptr::null(),
        )
    };

    if result != 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}
