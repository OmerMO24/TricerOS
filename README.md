# TricerOS

This is an operating system, kind of. I wouldn't call it a "minimal kernel" since it does support paging, has a virtual memory system, and consequentially
supports all of the modules that form rust's alloc crate (vec, string, str, sync etc). From everything I can find, minimal kernels only offer an abstraction to
write to the VGA text mode buffer. I guess you could call this a "half kernel".

Things that Tricer does not offer (yet):

 - Tricer does not have the notion of a user.
 - Tricer does not have syscalls.
 - Tricer does not have a File System.
 - Tricer does not support async rust.
 - Tricer does not have the notion of a process (by consequence Tricer does not have a scheduler).

As you can see, this is very much a work in progress. But thanks to the many operating systems developers before me, implementing these shouldn't be too much
of a problem. As for running Tricer on your own qemu instance, I haven't quite figured that out just yet, but I will upload instructions in due time.

Thanks for checking this out!  
