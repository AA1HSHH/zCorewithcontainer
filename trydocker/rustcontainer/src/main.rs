use nix::{sched,unistd,mount};
use nix::sys::wait::waitpid;
use nix::sys::signal::Signal;
use std::process::{self,Command};
use rtnetlink::new_connection;
use tokio;




#[tokio::main]
async fn main() {
    if !unistd::Uid::effective().is_root() {  //https://stackoverflow.com/questions/62517543/how-can-i-ensure-that-a-rust-executable-is-run-with-root-permissions-on-linux
        panic!("You must run this code with root permissions\n please use 'su' switch to root");
    }

    run().await;
    println!("bye bye");
}

async fn run() {
    println!("father pid is {}", process::id());

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);
    handle
        .link()
        .add()
        .veth("veth-rs-1".into(), "veth-rs-2".into())
        .execute()
        .await
        .map_err(|e| format!("{}", e)).expect("veth err");

    const STACK_SIZE: usize = 1024 * 1024;
    let stack: &mut [u8; STACK_SIZE] = &mut [0; STACK_SIZE];
	let cb = Box::new(|| child());
	let clone_flags =
		sched::CloneFlags::CLONE_NEWNS | sched::CloneFlags::CLONE_NEWPID | sched::CloneFlags::CLONE_NEWUTS | sched::CloneFlags::CLONE_NEWNET ;
	let child_pid = sched::clone(cb, stack, clone_flags, Some(Signal::SIGCHLD as i32))
		.expect("Failed to create child process");
    // TODO
    //handle.link().get().setns_by_pid(child_pid).execute().await.map_err(|e| format!("{}", e)).expect("setns err");
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
