package main

import (
	"fmt"
	"os"
	"os/exec"
	"syscall"
)

func main() {
	// We expect "run" as the first argument
	switch os.Args[1] {
	case "run":
		run()
	case "child":
		child()

	default:
		panic("Bad argument")
	}

	fmt.Println("== Finished ==")
}

func run() {
	// Arguments 2 onwards are the arbitrary command we're going to run
	fmt.Printf("Running1 %v\n , process ID %d\n", os.Args[2:], os.Getpid())

	// Set up a struct that describes the command we want to run
	args := os.Args
	args[1] = "child"
	cmd := exec.Command("/proc/self/exe", args[1:]...)

	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.SysProcAttr = &syscall.SysProcAttr{
		Cloneflags: syscall.CLONE_NEWUTS | syscall.CLONE_NEWPID | syscall.CLONE_NEWNS,
	}
	// This is where we run the command
	err := cmd.Run()
	if err != nil {
		panic(fmt.Sprintf("running: %v\n", err))
	}
}

func child() {
	// Arguments 2 onwards are the arbitrary command we're going to run
	fmt.Printf("Running2 %v\n , process ID %d\n", os.Args[2:], os.Getpid())
	// Set the hostname for the container

	// Set up a struct that describes the command we want to run
	cmd := exec.Command(os.Args[2], os.Args[3:]...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err := syscall.Sethostname([]byte("container"))
	if err != nil {
		panic(fmt.Sprintf("sethostname: %v\n", err))
	}
	// Change the root directory
	err = syscall.Chroot("alpine")
	if err != nil {
		panic(fmt.Sprintf("chroot: %v\n", err))
	}


	// Move into the new root directory
	err = syscall.Chdir("/")
	if err != nil {
		panic(fmt.Sprintf("chdir: %v\n", err))
	}

	// Mount the proc pseudo-filesystem
	err = syscall.Mount("proc", "proc", "proc", 0, "")
	if err != nil {
		panic(fmt.Sprintf("mymount: %v\n", err))
	}

	// This is where we run the command
	err = cmd.Run()
	if err != nil {
		panic(fmt.Sprintf("running123: %v\n", err))
	}

}
