use nix::{sched,unistd,mount};
use nix::sys::wait::waitpid;
use nix::sys::signal::Signal;

use std::process::{self,Command};



fn main() {
    if !unistd::Uid::effective().is_root() {  //https://stackoverflow.com/questions/62517543/how-can-i-ensure-that-a-rust-executable-is-run-with-root-permissions-on-linux
        panic!("You must run this code with root permissions\n please use 'su' switch to root");
    }

    run();
    println!("bye bye");
}

fn run() {
    println!("father pid is {}", process::id());
    const STACK_SIZE: usize = 1024 * 1024;
    let stack: &mut [u8; STACK_SIZE] = &mut [0; STACK_SIZE];
	
	let cb = Box::new(|| child());

	
	let clone_flags =
		sched::CloneFlags::CLONE_NEWNS | sched::CloneFlags::CLONE_NEWPID | sched::CloneFlags::CLONE_NEWUTS ;
	let child_pid = sched::clone(cb, stack, clone_flags, Some(Signal::SIGCHLD as i32))
		.expect("Failed to create child process");
	waitpid(child_pid, None).expect("waitpid error");
    
    

}

fn child() -> isize{
    
    //println!("child123");
    println!("child  pid is {}", process::id());
    


    sched::unshare(sched::CloneFlags::CLONE_NEWNS).expect("Failed to unshare");
    unistd::sethostname("container").expect("hostname err");
    unistd::chroot("/path/to/alpine").expect("chroot error");
    unistd::chdir("/").expect("chdir err");
    mount::mount(Some("proc"), "proc", Some("proc"), mount::MsFlags::empty() , Some("")).expect("mount err");
    let _sh = Command::new("sh")
    .status()
    .expect("fail");


    1

}
